# Asynchronous Support and Programming:

This article is relevant to Linux as a Operating system in general, covering how the operating system
supports Asynchronous IO programming model, along with languages that can utilize this asynchronous
programming model. 

### History Of Async I/O:
---

**1. ( Linux kernel < 2.6.x )** 
- Pre Linux kernel version 2.6.x only supported blocking I/O via ( `read()`/`write()` ), the only possible
  option for programmers to use async I/O was to use the POSIX AIO (`aio_read()` and `aio_write()`) which
  existed in user-space and exposed to programs via header. The Asynchronous behaviour was *emulated*
  through treads because there was no true kernel `async` handling. ( POSIX AIO used threads under the hood
  that fake asynchrony. )

- POSIX AIO: ( `glibc` ) : Uses a **thread pool** in user-space to perform standard blocking `read()` and
  `write()` calls.

- This was a limitation to applications that do not use threads. 

**2. ( Linux kernel 2.6.x )**

- Native Linux AIO : ( via `libaio` ): 
    - Kernel level implementation using `io_submit()` and `io_getevents()`.
    These implementations are still in the new kernels and should be the contention for eBPF internal study. 
    To understand AIO we can split this into two parts: user and kernel spaces:

- User-Space side: 
    - AIO mechanism allows user-space process to initiate multiple I/O operations without waiting for any of
      them to complete.
      
      Unlike standard I/O which blocks the program, AIO lets the program move on to the task immediately,
      receiving a notification later when the data is ready.

    - AIO ( `libaio` ) operates on "submit and poll"  model. Instead of standard `read()` and `write()`
      system calls, it uses a specialized set of system calls to interact with the kernel's I/O queue.
      
      1. Setup (`io_setup`): Procecss requests the kernel to create an *AIO Context*, this is essentially a
         queue in kernel space that will track the state of your ongoing requests.

      2. Submission ( `io_submit` ): The process prepares one or more *I/O Control Blocks* (`iocb`). These
         blocks contain the file descriptors, the buffer address, and the offset. When `io_submit` is
         called:
         - Kernel adds these requests to the device's dispatch queue. 
         - The system call returns immediately.
         - The disk controller handles the actual data transfer in the background via DMA. 
      
      3. Completion and Harvesting ( `io_getevents` ): Once the hardware finishes, the kernel places a
         "completion event" into a ring buffer.  

         The user-space process calls `io_getevents` to check that buffer. 
         If the work is done, it retrieves the results; if not, it can choose to wait or go do more work.

    - How to use AIO ( Code ) : you will need `libaio` library. 

      1. Initialize the Context: 
      ```c 
      struct io_context *ctx = 0;
      int max_events = 128;
      io_setup(max_events, &ctx);
      ```

      2. Prepare the Request: You define what you want to do (e.g Read) in an `iocb` struct:
      ```c 
      struct iocb cb;
      struct iocb *cbs[1];
      chat *buffer = malloc(4096);

      io_prep_pread(&cb, fd, buffer, 4096, 0); // Read 4096 bytes at offset 0
      ```

      3. Submit and Reap:
      ```c 
      io_submit(ctx, 1, cbs); // Non-blocking call 

      // ... do other high-performance calculations here ...

      struct io_event events[1];
      io_getevents(ctx, 1, 1, events, NULL); // Wait for at least 1 event
      ```
---
Note: When to use AIO:

- Note AIO can not be used for every thing and there are strict requirements.
    * *O_DIRECT* is mandatory: File must be opened with *O_DIRECT* flag. Or else kernel will use page cache,
      and the AIO call will actually block while waiting for the cache to update, defeating the purpose.
    * *Filesystem Support*: Not all File systems support AIO, XFS and EXT4 its most efficient for large
      block operations.
    * The Modern Rival: As from kernel 5.1+, `io_uring` has largely replacecd `libaio`. Its faster, supports
      buffered I/O and has much cleaner interface.
---


- Kernel Side implementation: ( 2.6.0+ ) `fs/aio.c` 

    - To understand we have to look at the *AIO Context* and *I/O Scheduler*.

    1. **The AIO Context** ( `lioctx` )
    When you call `io_setup` kernel creates a `kioctx` structure for your process. This is private "mailbox"
    in kernel memory. ( i.e `kioctx` struct holds messages/requests that are sent from user-space to kernel)
    - **The Ring buffer*: Kernel maps a piece of memory that both kernel and application can see, this
      buffer stores *Completion Events*.
    - **Queue**: It sets up a tracing mechanism for "in-flight" requests so the kernel knows how many
      operations are currently touching the HW.

    2. **Submission: The fast path**: When you call `io_submit` the kernel does not start reading. It
       performs a high-speed handoff:
       - *Validation*: kernel checks `iocb` ( I/O control block ) is valid and file was opened with
         *O_DIRECT* flag.
       - *Mapping*: It pins the user-space memory pages. Because AIO is asynchronous, the kernel must ensure
         your app does not move or delete the memory buffer while the disk is writing to it.
       - *The Non-blocking Plug*: Instead of calling std read function that sleeps, it calls the
         `file_operations->read_iter` or `aio_read` method of the specific filesystem ( like XFS or EXT4 ).
       - *Immediate Return*: If the driver confirms the request is queued, the kernel immediately returns
         control to your program. Your CPU is now free to do other things.

    3. **DMA**: Kernel tells Disk Controller: "When you are ready take data from this sector and put it
       directly into this memory address. Do not bother the CPU until you finished". CPU is effectively
       removed from the data transfer process. Disk HW handles the heavy lifting in the background.

    4. **Completion The Interrupt**: Below is how the kernel knows its done.
       - HW Interrupt: disk controller sends an interrupt signal to the CPU.
       - Interrupt Handler: kernel's interrupt handler catches this and identifies which AIO request is
         finished.
       - Pushing the Event: The kernel writes a "success" (or error) message into the Ring Buffer we 
         mentioned in step 1.
       - Wake Up: If the user process was specifically waiting via `io_getevents`, the kernel wakes it up. 
         Otherwise, the event just sits in the buffer until the app decides to check it.

- Kernel manages "AIO" through a **Context** ( `kioctx`).

    For eBPF programs, these structures are the primary targets for `kprobe` and `fentry`.

    1. AIO Context ( `struct kioctx` ): This struct anchors the asynchronous life cycle to a process's memory
      map ( `mm_struct`):

      ```c 
      struct kioctx {
        atomic_t            users;
        struct mm_struct    *mm;        // Ties AIO to the process address space. 
        unsigned longuct    user_id;    // The Handle thats returned to user-space 
        wait_queue_head_t   wait;       // Used by `io_getevents` to sleep 
        struct list_head    active_reqs;// Linked list of in-flight kiocb 
        struct aio_ring_info ring_info; // The shared completion ring 
      };
      ```
      - `struct mm_struct   *mm;` this ties AIO context to a specific process memory space.
      -  Completion ring lives in user-space memory.
      - kernel maps pages into user-space.
      - Note submissionon is still "syscall-based".
      - Not lockless.
      - `struct mm_struct   *mm;` this ties AIO context to a specific process memory space.
      -  Completion ring lives in user-space memory.
      - kernel maps pages into user-space.
      - Note submissionon is still "syscall-based".
      - Not lockless.

    2. The I/O Control Block ( `struct kiocb` ): Each individual request is tracked via a `kiocb`
    In eBPF, you often trace this to measure latency (start time at `io_submit` vs. `end` time at `aio_complete`).

    ```c 
    struct kiocb {
        struct file         *ki_filp;
        struct kioctx       *ki_ctx;
        struct list_head    ki_list;       // Link to kioctx->active_reqs
        __u64                ki_user_data; // Opaque cookie (returned in io_event)
        loff_t               ki_pos;       // File offset
    };
    ```

    3. The Completion Ring (`aio_ring_info`)

    ```c 
    struct aio_ring_info {
        unsigned long       mmap_base;
        struct page         **ring_pages;      // Array of pages
        unsigned            nr, tail;          // Ring state
    };
    ```
- `ring_pages`: completion queue is allocated by kernel and  mmap's into user-space.

  The kernel allocates pages for the completion queue and maps them directly into user-space.

  Asymmetry: 
    Submission is a syscall (heavy); Completion is a memory-mapped read (light).

  IRQ Context: 
    The kernel writes to this ring during the Interrupt Request (IRQ) handler via `aio_complete()`. 

NOTE: `eBPF`:
---
To trace Native AIO on modern Linux (6.X.+):
1. The Entry Hook: trace `io_submit_one`, this function takes `struct kioctx *ctx` and 
   a `struct iocd __user   *user_iocb`.

   - eBPF Task: Read the user-space `iocb` to see what the application wants to do.

2. The completion Hook: Trace `aio_complete`. This function takes a `struct aio_kiocb *iocb`
   - eBPF task: Pull the timing information. Since you have the pointer to the `aio_kiocb`. You can use it
     as a "key" in a BPF map to calculate how long that specific request took from submission to completion.

To study AIO in modern linux perforamnce, look into `io_uring`. Its essentially "AIO 2.0"
- *Native AIO* ( `libaio` ) uses `struct aio_kiocb`.
- *io_uring*: uses `struct io_kiocb`.

`io_uring` is much faster as it avoids the system call overhead by using shared memory rings.
---

Critical limitation encoded in 2.6.0:

```c
int io_submit_one(/*...*/) {
    file = fget(iocb->aio_fildes);
    req = aio_get_req(ctx);   // kmem_cache_alloc PER I/O
    
    switch (iocb->aio_lio_opcode) {
    case IOCB_CMD_PREAD:
    case IOCB_CMD_PWRITE:
        /* O_DIRECT CHECK MISSING — happens later in VFS */
        ret = file->f_op->aio_read(req, ...);
        // If not O_DIRECT: blocks in io_submit
    }
}
```

This gives:
` kernel => Writes completion => user space reads ring`
but submission still depends on `iocb`.
This asymmetry is important. 

What the 2.6.0 code actually guarantees:

- ✅ Per-context ring buffer (`io_event` queue)
- ✅ No kernel threads; completions arrive via IRQ -> `aio_complete()`
- ❌ **No protection against blocking `io_submit`**
- ❌ No buffered I/O support (silently falls back to sync)
- ❌ No socket/pipe support


### The "False Promise" of 2.6 AIO

Three "Gotchas" that often break AIO asynchronicity:

| Limitation | Technical Reason |
| --- | --- |
| **Blocking `io_submit**` | If the file is not opened with `O_DIRECT`, the kernel falls back to synchronous buffered I/O. `io_submit` won't return until the data is in the page cache. |
| **Metadata Bottleneck** | Even with `O_DIRECT`, if the filesystem needs to allocate blocks (ext4/XFS), the `io_submit` call will block on disk I/O for metadata updates. |
| **Allocation Overhead** | `aio_get_req` calls `kmem_cache_alloc` for **every** I/O. This creates significant slab pressure during high-throughput bursts. |

---
### Userspace: `libaio` vs. Reality

While `libaio` provides a wrapper for the syscalls, the developer must follow strict rules to avoid 
"Accidental Synchronicity."

The `O_DIRECT` Requirement:

Without `O_DIRECT`, Linux AIO is essentially useless, as it behaves like a std blocking call but with more overhead.

```c
// Best practice circa 2006
int fd = open("data.bin", O_DIRECT | O_RDWR);
if (fd < 0) perror("O_DIRECT not supported");

// Memory must be page-aligned for O_DIRECT
void *buf;
posix_memalign(&buf, 512, 4096); 

```

Submission Example:

```c
struct iocb cb;
struct iocb *cbs[1] = { &cb };

io_prep_pwrite(&cb, fd, buf, 4096, 0);
cb.data = (void *)0xDEADBEEF; // User-defined ID for tracking

int ret = io_submit(ctx, 1, cbs); 
// If ret != 1, submission failed or blocked.

```

### Key Takeaways for eBPF Integration

* Tracing Submission: 
    Probe `sys_io_submit`.
    Check if the process spends significant time in "D" (uninterruptible sleep) state here—this 
    indicates `O_DIRECT` or metadata blocking.

* Tracing Completion:
    Probe `aio_complete`. This is where the kernel finishes the work. 
    The time delta between `io_submit` and `aio_complete` is your true disk I/O latency.

---

## Phase 2 — Linux Native AIO (`io_submit`, `io_getevents`) (~Linux 2.6/3.x)

### 1. The Kernel Execution Path

When an application calls `io_submit()`, the kernel doesn't always go asynchronous. 

For an eBPF developer, the `vfs_read` and `vfs_write` layers are where the "magic" (or the blocking) 
happens.

* The O_DIRECT Fast Path:
    The kernel checks the `O_DIRECT` flag. 
    If present, it maps the user-space buffer directly to the DMA (Direct Memory Access) engine of the
    storage controller. 
    The syscall returns **immediately** while the hardware does the work.

* The Buffered Fallback (The "Silent Killer"):
    If `O_DIRECT` is missing, the kernel attempts to use the Page Cache. 
    Because the Page Cache logic in this era was fundamentally synchronous, the `io_submit` syscall 
    would **block** until the data was copied into the cache, defeating the purpose of AIO.

---

### 2. The "Double-Syscall" Tax

Even in a perfect scenario (using `O_DIRECT`), Phase 2 AIO suffered from high overhead due to the way 
userspace and kernel-space interacted:

| Step | Action | Overhead Type |
| --- | --- | --- |
| **Submission** | `io_submit()` | Context Switch + Parameter Copying |
| **Execution** | Hardware I/O | (Background) |
| **Harvesting** | `io_getevents()` | Context Switch + Event Copying |

**The eBPF Perspective:** If you were to write a BCC/bpftrace tool here, you would see a high frequency of 
`switch_to` calls. 

Each I/O operation requires at least two transitions across the user/kernel boundary.

---

### 3. Deep Dive: Why `O_DIRECT` is Picky

"HW becomes picky." this is because the **Memory Management Unit (MMU)** and the **Disk Controller** need 
to talk directly.

* Alignment:
    Disk sectors are physical blocks (512B or 4KB). 
    The DMA engine cannot "offset" a transfer by 7 bytes; it needs to point to a memory address that 
    aligns with the hardware's physical architecture.

* Zero-Copy:
    `O_DIRECT` is the closest thing to "Zero-Copy" I/O in this era. 
    The kernel does not touch the data; it simply facilitates the handshake between the RAM and the Disk.

---

### 4. Summary of Phase 2 Limitations (The "Why" for io_uring)

| Limitation | Impact on Application |
| --- | --- |
| **No Socket Support** | You couldn't use `io_submit` for network programming (epoll was still required). |
| **Metadata Blocking** | Even with `O_DIRECT`, if the file size changed, the kernel blocked to update the inode. |
| **Fixed Ring Size** | `io_setup` defines a fixed capacity; if you exceed it, `io_submit` fails or blocks. |


The **asymmetry of the API** (syscall for submission vs. syscall for harvesting). 
This is the "performance wall" that eventually led to the creation of `io_uring`.


### 5. Userspace Side — `libaio`

* `libaio` exposed the native AIO API.
* Allowed non-blocking submission of I/O to kernel.

User-space:

```c
io_context_t ctx = 0;
io_setup(128, &ctx);
struct iocb *cbs[...];
io_submit(ctx, n_requests, cbs);
struct io_event events[n_requests];
io_getevents(ctx, min, max, events, NULL);
```

**Limitations in User-space:**

* Mandatory "O_DIRECT" semantics complicate application logic.
* Separate submission + completion syscalls mean syscall and context-switch overhead.

**Adoption:**

* Some databases and app servers adopted it (e.g., MySQL InnoDB’s native AIO) ([Oracle Docs][5])
* But the complexity and limited benefit for buffered I/O discouraged widespread use.
---

### Pro-Tip for eBPF:

When you get to the coding part of your tutorial, a great exercise would be:

> "Write a `kprobe` on `aio_rw_done` to see how long the kernel spends in the interrupt handler versus 
the time the user-space spends waiting in `io_getevents`."


## Phase 3: The Revolution: `io_uring` (Linux 5.1+)

### KERNEL SIDE: The New Shared-Memory Contract

By 2019, the kernel community acknowledged that AIO (Phase 2) was fundamentally broken for most modern use 
cases. 

As the *Linux Kernel Newbies* notes for the 5.1 release:

> "Linux has had an asynchronous I/O interface for a long time... [It] has historically suffered from a large number of shortcomings. It does not support buffered I/O... All attempts to fix the existing interface have failed."

The Solution: 
    Instead of passing data through syscalls, kernel and userspace now share **Circular Buffers (Rings)**.

#### The Data Structures (`io_uring.h`)

Unlike the `kiocb` of Phase 1/2, these structures are designed to be "cache-line friendly" and reside in 
memory mapped by both the kernel and the app.

```c
struct io_uring_sqe {
    __u8    opcode;      // IORING_OP_READ, IORING_OP_WRITE, IORING_OP_OPENAT...
    __s32    fd;          // File descriptor
    __u64    off;         // Offset
    __u64    addr;        // Buffer address
    __u32    len;         // Buffer size
    __u64    user_data;   // Opaque ID (returned in CQE)
    __u8     flags;       // IOSQE_FIXED_FILE, IOSQE_IO_LINK...
    /* ... unions for 50+ opcodes ... */
};

struct io_uring_cqe {
    __u64    user_data;   // Matches SQE->user_data
    __s32    res;         // Result (bytes read or -errno)
    __u32    flags;
};

```

#### What `io_uring` actually delivers:

1. True Asynchrony for Buffered I/O:
    Unlike Phase 2, if a file is not `O_DIRECT`, `io_uring` hands the work to internal kernel worker
    threads (`io-wq`) instead of blocking the submission.

2. Syscall Elision (SQPOLL):
    You can tell the kernel to create a thread that **polls** the submission ring. 
    The app just writes to memory—**zero syscalls** are required to send I/O.

3. Submission/Completion Decoupling:
    One syscall (`io_uring_enter`) can submit 100 requests and harvest 100 completions simultaneously.

4. Opcode Diversity:
    It isn't just for disk I/O; it supports `accept()`, `send()`, `recv()`, and even `splice()`, finally
    bringing AIO to networking.

---

### USER-SPACE SIDE: `liburing`

Directly managing memory barriers and ring offsets is dangerous. Jens Axboe (the creator of `io_uring`) 
released `liburing` to simplify the "contract."

```c
/* 1. Setup: one mmap() call, not per-I/O allocations */
struct io_uring ring;
io_uring_queue_init(QUEUE_DEPTH, &ring, 0);

/* 2. Submission: Prep data in the ring */
struct io_uring_sqe *sqe = io_uring_get_sqe(&ring);
io_uring_prep_read(sqe, fd, buf, sizeof(buf), 0);
io_uring_sqe_set_data(sqe, my_custom_ptr);

/* 3. The "Enter" call: Notifies kernel of new work */
io_uring_submit(&ring);

/* 4. Completion: No syscall needed to check the ring */
struct io_uring_cqe *cqe;
io_uring_wait_cqe(&ring, &cqe);
/* Process cqe->res */
io_uring_cqe_seen(&ring, cqe);

```

---

### The Evolution Summary (The "Why" for eBPF)

When you start writing your eBPF programs later in this tutorial, keep this progression in mind:

| Phase | Trace Point | eBPF Target | Challenge |
| --- | --- | --- | --- |
| **Phase 1** | Thread creation | `kprobe:do_fork` | High overhead, hard to link thread to I/O. |
| **Phase 2** | Syscall entry/exit | `tracepoint:syscalls:sys_enter_io_submit` | Blocking behavior in the "Submit" phase. |
| **Phase 3** | Tracepoints | `tracepoint:io_uring:io_uring_submit_sqe` | **SQPOLL** mode bypasses syscalls; you *must* use tracepoints. |

### Note on "Linked Operations":

A unique feature of Phase 3 is the ability to chain operations. 
You can tell the kernel: "Open this file, THEN read it, THEN close it," all in one submission. 

In eBPF, this looks like a sequence of events triggered from a single `io_uring_enter`.



## PHASE 4: 2020–2025: The Adoption War

### KERNEL SIDE: Feature Blitz

While Phase 3 gave us the architecture, Phase 4 turned `io_uring` into a Swiss Army Knife. 
The interface expanded from simple file reads to handling the entire system lifecycle without ever leaving
the ring.

The 2025 "Universal" SQE (`io_uring.h`):

By 2025, the `io_uring_sqe` union has matured to support ~50+ opcodes. 
This is no longer just for I/O; it is for **synchronization and hardware passthrough**.

```c
/* Current state of the SQE union (circa 2025) */
union {
    __kernel_rwf_t     rw_flags;        // Standard readv/writev
    __u32                poll32_events; // IORING_OP_POLL_ADD
    __u32                msg_flags;     // IORING_OP_SEND_ZC (Zero-Copy)
    __u32                accept_flags;  // IORING_OP_ACCEPT (Network)
    __u32                uring_cmd_flags; // IORING_OP_URING_CMD (NVMe Passthrough)
    __u32                futex_flags;   // IORING_OP_FUTEX_WAIT (6.4+)
};

```

**Key Technological Leaps:**

* IORING_OP_URING_CMD (5.19/6.0):
    This allows "Big SQEs" (128 bytes). 
    It lets apps talk directly to NVMe hardware, bypassing the entire block layer. 
    In eBPF, you’d trace this via `nvme_process_user_cmd`.

* IORING_OP_FUTEX_WAIT (6.4):
    Perhaps the most radical addition. 
    You can now put a thread to sleep or wake it up via the ring, meaning `io_uring` is now a **concurrency
    primitive**, not just an I/O one.

* Zero-Copy (6.0+):
    `IORING_OP_SEND_ZC` allows the kernel to send network packets without a memory copy, notifying the app
    via the CQE when the buffer is safe to reuse.

---

### USERSPACE SIDE: The Breaking Point

As of **PostgreSQL 18 (Released late 2024/2025)**, the industry is finally hitting the "Adopt or Die"
moment. However, the integration isn't easy because `io_uring` requires a total rethink of application
architecture.

The PostgreSQL Challenge (from March 2025 commits):

PostgreSQL traditionally uses a process-per-backend model. This creates a friction point with `io_uring`’s 
"Ring-Per-Thread" philosophy.

```c
/* method_io_uring.c - March 2025 Implementation */

/* * Issue: Creating one io_uring per backend is fast, but 
 * sharing rings across backends would require locking,
 * which kills the 'lockless ring' performance benefit.
 */
if (io_method == IO_METHOD_IO_URING) {
    // Requires --with-liburing at compile time
    // Current limitation: Used primarily for READS; 
    // index scans and writes still evolving.
}

```

The Reality of 2025 Adoption:

* Postgres 18:
    Introduces `io_method=io_uring` but defaults to `worker` (thread-pool) for stability across non-Linux
    platforms.

* The Security Block:
    Many container environments (Docker/K8s) **block `io_uring` by default** via `seccomp`. 
    This is because `io_uring` has such a massive attack surface—it can essentially execute any syscall
    from a memory buffer, making standard seccomp filters blind.
---

### COMPLETE TIMELINE (KERNEL + USERSPACE)

| Era | Kernel State | Userspace State | The "Gap" |
| --- | --- | --- | --- |
| **Phase 1 (2.6.0–2.6.18)** | `fs/aio.c` ships. No `O_DIRECT` enforcement. | `libaio` exists but is "fake" async. | **The Lie:** Apps thought it was async; kernel blocked them. |
| **Phase 2 (2.6.19–4.20)** | Stagnation. No core AIO fixes. | `epoll` + `eventfd` hacks dominate. | **The Workaround:** Developers gave up on AIO and built `epoll` instead. |
| **Phase 3 (5.1 - 2019)** | **Revolution:** `io_uring` merged. Shared rings. | `liburing` simplifies the contract. | **The Reset:** AIO is effectively deprecated for new projects. |
| **Phase 4 (2020–2025)** | **Expansion:** 50+ opcodes. NVMe Passthrough & Futexes. | Major DBs (Postgres 18) adopt it. | **The Battle:** Performance is ready, but Security (Seccomp) is afraid. |

---

### Transition to eBPF: 

When we write our eBPF programs below, we aren't just tracing functions; we are hunting for the 
**transition points** across these eras:

1. We will use eBPF to detect if a process is "stuck" in a **Phase 2 `io_submit**` call (Blocked I/O).
2. We will use eBPF to trace **Phase 4 `io_uring**` activity even when `strace` can't see it (because of SQPOLL).
3. We will measure the latency difference between the **Postgres `worker` method** (Phase 1-style) and the **`io_uring` method** (Phase 4).


## Tracing the Untraceable

### 1. The "Silent" Era (2.6.22–5.0)

**2.6.0**: `struct kioctx`, `struct kiocb`, `struct aio_ring_info` – frozen. 

Note there is a lack of commits. This wasn't because work stopped; it was because the kernel community was
trying to fix AIO's fundamental flaw: **the Page Cache**.

* The Problem:
    The Page Cache is a synchronous state machine. 
    Making `read()` truly async for buffered I/O required re-architecting how the kernel handles
    "page faults" during I/O—a task so hard it took a decade.

* The Traceable Event (2.6.22):
    The addition of `eventfd`. This is the "missing link." It allowed AIO completions to be signaled into
    an `epoll` loop.

* eBPF Target: 
    `kprobe:eventfd_signal`. 
    If you see this firing after an `io_submit`, you are looking at the Phase 2 "Workaround" era.


### 2. The Opcode Explosion (2019–2025)

The manpage lists 50+ opcodes, but the *logic* of how they were added is what matters for eBPF.

* The Pattern:
    Every time a new opcode (like `IORING_OP_SEND_ZC`) is added, a corresponding **Tracepoint** is usually 
    added to `include/trace/events/io_uring.h`.

* Why this matters:
    You don't need to know the commit hash for every opcode. You only need to run:
    `perf list | grep io_uring`

```
$ sudo perf list | grep io_uring
io_uring:io_uring_complete                         [Tracepoint event]
  io_uring:io_uring_cqe_overflow                     [Tracepoint event]
  io_uring:io_uring_cqring_wait                      [Tracepoint event]
  io_uring:io_uring_create                           [Tracepoint event]
  io_uring:io_uring_defer                            [Tracepoint event]
  io_uring:io_uring_fail_link                        [Tracepoint event]
  io_uring:io_uring_file_get                         [Tracepoint event]
  io_uring:io_uring_link                             [Tracepoint event]
  io_uring:io_uring_local_work_run                   [Tracepoint event]
  io_uring:io_uring_poll_arm                         [Tracepoint event]
  io_uring:io_uring_queue_async_work                 [Tracepoint event]
  io_uring:io_uring_register                         [Tracepoint event]
  io_uring:io_uring_req_failed                       [Tracepoint event]
  io_uring:io_uring_short_write                      [Tracepoint event]
  io_uring:io_uring_submit_req                       [Tracepoint event]
  io_uring:io_uring_task_add                         [Tracepoint event]
  io_uring:io_uring_task_work_run                    [Tracepoint event]
  syscalls:sys_enter_io_uring_enter                  [Tracepoint event]
  syscalls:sys_enter_io_uring_register               [Tracepoint event]
  syscalls:sys_enter_io_uring_setup                  [Tracepoint event]
  syscalls:sys_exit_io_uring_enter                   [Tracepoint event]
  syscalls:sys_exit_io_uring_register                [Tracepoint event]
  syscalls:sys_exit_io_uring_setup                   [Tracepoint event]
```
This will show you every "hook" the current kernel provides for that specific version.

---

### 3. Solving the "Un-traceable" Problems

#### A. The SQPOLL Problem (No Syscalls)

If PostgreSQL uses `IORING_SETUP_SQPOLL`, `strace` will show **nothing**—no `io_uring_enter` calls. 
The app just writes to RAM.

* The Solution: You must trace the **Kernel Thread**.
* eBPF Strategy: 
    Use `kprobe` on `io_sq_thread`. 
    This thread is the one actually doing the work. 
    You can measure its CPU usage to see how much "background" I/O processing is happening that `top` 
    might miss.

#### B. The Container/Seccomp Block

seccomp blocks `io_uring`. This is the "Adoption War" of 2025.

* The Technical Reason:
    Seccomp filters syscalls at the boundary. 
    But `io_uring` is a "Syscall Multiplexer."    One `io_uring_enter` syscall could be a `read`, a
    `write`, or an `open`. Seccomp can't see "inside" the ring buffer.

* The eBPF Solution (2025 State of the Art): 
    The kernel is moving toward **LSM (Linux Security Module) hooks for io_uring**.

* New Hook:
    `security_uring_cmd`.

* eBPF Program:
    You can now write an `LSM` BPF program to inspect the SQEs *inside* the ring before the kernel 
    executes them. This is how you "fix" the security problem that seccomp couldn't solve.


---

### 4. Updated Complete Timeline (The "Gap-Filled" Version)

| Era | Notable "Silent" Event | eBPF Hook | Why it matters |
| --- | --- | --- | --- |
| **2007** | `eventfd` integration | `eventfd_signal` | The only way Phase 2 AIO could talk to `epoll`. |
| **2012** | GlusterFS / AIO Latency Fixes | `kprobe:aio_complete` | Proved that AIO was mostly "blocking in disguise." |
| **2019** | `io_uring` 5.1 Merge | `tracepoint:io_uring:io_uring_submit_sqe` | The birth of high-performance tracing. |
| **2022** | **IORING_OP_BPF** (Proposal) | `IORING_OP_BPF` | Experimental opcodes to run BPF *inside* the ring. |
| **2025** | **LSM BPF** Security Hooks | `lsm/uring_cmd` | Solving the "Seccomp" block in Docker/K8s. |

---

Flow:
1. **2.6.0:** The "Frozen" structures are your foundation.
2. **2.6.0–Present:** The "Blocking" behavior is the villain of the story.
3. **2012:** Userspace takes the blame for alignment (the `O_DIRECT` struggle).
4. **2019:** The "Revolution" provides the new architecture.
5. **2025:** The "Adoption War" where eBPF becomes the only way to secure and observe the ring.

## Rust eBPF code ( illustrate the adoption war )


Example : The programs target the `io_uring_enter` syscall (the heart of Phase 3/4) to observe the 
**latency** and **frequency** of asynchronous requests.


### Rust Solution: Total Aya Program

Aya projects are split into a **workspace**. We will focus on the two main components: 
the kernel probe (`ebpf`) and the loader/logger (`userspace`).

#### 1. Kernel-Space (Rust eBPF)

Located at `my-project-ebpf/src/main.rs`. This code runs inside the kernel.

```rust
#![no_std]
#![no_main]

use aya_ebpf::{
    macros::kprobe,
    programs::ProbeContext,
    helpers::bpf_get_current_pid_tgid,
};
use aya_log_ebpf::info;

#[kprobe]
pub fn trace_io_uring_enter(ctx: ProbeContext) -> u32 {
    match try_trace_io_uring_enter(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_trace_io_uring_enter(ctx: ProbeContext) -> Result<u32, u32> {
    // Get the PID of the process entering the ring
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    
    // Log to the userspace logger
    info!(&ctx, "PID {} is entering io_uring", pid);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

```

#### 2. User-Space (Rust Loader)

Located at `my-project/src/main.rs`. This loads the program and prints logs.

```rust
use aya::{include_bytes_aligned, Ebpf};
use aya::programs::KProbe;
use aya_log::EbpfLogger;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load the eBPF object file
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/my-project"
    ))?;

    // Initialize logger to receive info!() messages from kernel
    EbpfLogger::init(&mut bpf)?;

    // Attach the kprobe to the io_uring_enter syscall
    let program: &mut KProbe = bpf.program_mut("trace_io_uring_enter").unwrap().try_into()?;
    program.load()?;
    // We target the kernel function responsible for the syscall
    program.attach("__x64_sys_io_uring_enter", 0)?;

    println!("Waiting for io_uring events... Press Ctrl-C to exit.");
    signal::ctrl_c().await?;

    Ok(())
}

```

---

### C Comparison: Kernel-Space Only

This is the "Legacy" way of writing the same kernel logic. 
It is more verbose regarding memory access but often easier to read for those coming from a Linux 
internals background.

```c
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

// This macro defines the section name for the loader
SEC("kprobe/__x64_sys_io_uring_enter")
int bpf_trace_uring(struct pt_regs *ctx) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;

    // The C equivalent of info!() is bpf_printk
    // You can view this via: sudo cat /sys/kernel/debug/tracing/trace_pipe
    char fmt[] = "C-PROBE: PID %d entering uring\n";
    bpf_trace_printk(fmt, sizeof(fmt), pid);

    return 0;
}

char _license[] SEC("license") = "GPL";

```

---

### Critical Comparison 

| Feature | Rust (Aya) | C (Libbpf) |
| --- | --- | --- |
| **Safety** | Uses `Result` and `match` to handle errors gracefully. | Uses raw return codes (0, -1) and pointers. |
| **Logging** | `aya-log` provides structured, type-safe logging. | `bpf_printk` is a simple string formatter. |
| **Compilation** | Integrated into `cargo`. No C toolchain needed. | Requires `clang`, `llvm`, and kernel headers. |
| **Data Types** | Shared via `common` crate (Rust to Rust). | Shared via headers (`.h` files). |

### Why this fits Phase 4:

In 2025, tools like **PostgreSQL** are moving fast. 
If you use the **C approach**, your tracing code might break when kernel structures change. 
If you use **Rust/Aya**, you can leverage Rust's strong type system and Aya's BTF (BPF Type Format) 
support to ensure your "Adoption War" monitoring tools are stable across different Linux distributions.


##  Comparison for Phase 4:
To complete the comparison for **Phase 4**, we will add a **HashMap** to the project. 
This map will store the number of `io_uring_enter` calls made by each Process ID (PID).

In the "Adoption War," this allows you to see which backends (like PostgreSQL) are aggressively using the
ring and which are falling back to legacy paths.

---

### Rust Solution: Total Aya Program with Maps

#### 1. Kernel-Space (Rust eBPF)

We use the `#[map]` macro to define a `HashMap` that the kernel can update.

```rust
// my-project-ebpf/src/main.rs
#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{kprobe, map},
    maps::HashMap,
    programs::ProbeContext,
    helpers::bpf_get_current_pid_tgid,
};

#[map]
static mut COUNTERS: HashMap<u32, u64> = HashMap::with_max_entries(1024, 0);

#[kprobe]
pub fn trace_io_uring_enter(ctx: ProbeContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;

    // Use a pointer to update the value in-place safely
    unsafe {
        if let Some(count) = COUNTERS.get_ptr_mut(&pid) {
            *count += 1;
        } else {
            // Initializing with 1 if PID is seen for the first time
            let _ = COUNTERS.insert(&pid, &1, 0);
        }
    }
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

```

#### 2. User-Space (Rust Loader)

The loader now periodically reads from the map and prints the leaderboard.

```rust
// my-project/src/main.rs
use aya::maps::HashMap;
use aya::{include_bytes_aligned, Ebpf};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/my-project"
    ))?;

    // Attach kprobe as before
    let program: &mut aya::programs::KProbe = bpf.program_mut("trace_io_uring_enter").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_io_uring_enter", 0)?;

    // Access the shared Map
    let mut counters: HashMap<_, u32, u64> = HashMap::try_from(bpf.map_mut("COUNTERS").unwrap())?;

    println!("Top io_uring users:");
    let mut interval = time::interval(Duration::from_secs(2));

    loop {
        interval.tick().await;
        // Iterate through all PIDs in the map
        for result in counters.iter() {
            let (pid, count) = result?;
            println!("PID: {} | Requests: {}", pid, count);
        }
    }
}

```

---

### C Comparison: Kernel-Space with Maps

In C, maps are defined in a special `.maps` section. This is the syntax you would see in traditional tutorials or the Linux kernel source itself.

```c
// trace_uring.bpf.c
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, u32);   // PID
    __type(value, u64); // Count
} counters SEC(".maps");

SEC("kprobe/__x64_sys_io_uring_enter")
int bpf_trace_uring(struct pt_regs *ctx) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    u64 *count;
    u64 one = 1;

    count = bpf_map_lookup_elem(&counters, &pid);
    if (count) {
        // Atomic increment to prevent race conditions across CPUs
        __sync_fetch_and_add(count, 1);
    } else {
        bpf_map_update_elem(&counters, &pid, &one, BPF_ANY);
    }

    return 0;
}

char _license[] SEC("license") = "GPL";

```

---

### Understanding the Comparison

| Concept | Rust (Aya) Implementation | C (BPF) Implementation |
| --- | --- | --- |
| **Map Definition** | `static mut COUNTERS: HashMap<...>` | `struct { ... } counters SEC(".maps")` |
| **Lookup** | `COUNTERS.get_ptr_mut(&pid)` | `bpf_map_lookup_elem(&counters, &pid)` |
| **Concurrency** | Handled by `get_ptr_mut` and raw pointer deref. | Handled by `__sync_fetch_and_add`. |
| **Memory Management** | Safe wrappers around pointers. | Raw pointers and manual null-checks. |

### Why this is the "2025 Standard":

By Phase 4, your monitoring tools need to be as efficient as the apps they monitor. Using `HashMap` allows you to aggregate data in the kernel so that your user-space tool only wakes up once every few seconds to print results, rather than processing a stream of every single event.

**Next Step**
Look at how to **bypass Seccomp** using the new 2025 eBPF LSM hooks we discussed, or would you like to 
refine the **PostgreSQL-specific** tracing logic:


## eBPF project for Phase 4.(evolution)

### Rust Solution: Total Aya Program

1. Kernel-Space (`ebpf/src/main.rs`)
We use a `HashMap` to track the "Adoption War" by counting per-PID `io_uring` activity.

```rust
#![no_std]
#![no_main]

use aya_ebpf::{macros::{kprobe, map}, maps::HashMap, programs::ProbeContext, helpers::bpf_get_current_pid_tgid};

#[map]
static mut COUNTERS: HashMap<u32, u64> = HashMap::with_max_entries(1024, 0);

#[kprobe]
pub fn trace_io_uring(ctx: ProbeContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    unsafe {
        let count = COUNTERS.get_ptr_mut(&pid).unwrap_or_else(|| {
            COUNTERS.insert(&pid, &0, 0).ok();
            COUNTERS.get_ptr_mut(&pid).unwrap()
        });
        *count += 1;
    }
    0
}

```

**2. User-Space (`src/main.rs`)**
The loader reaps the leaderboard every 2 seconds.

```rust
use aya::maps::HashMap;
use aya::{include_bytes_aligned, Ebpf, programs::KProbe};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut bpf = Ebpf::load(include_bytes_aligned!("../../target/bpfel-unknown-none/debug/project"))?;
    let program: &mut KProbe = bpf.program_mut("trace_io_uring").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_io_uring_enter", 0)?;

    let mut counters: HashMap<_, u32, u64> = HashMap::try_from(bpf.map_mut("COUNTERS").unwrap())?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        for res in counters.iter().filter_map(|r| r.ok()) {
            println!("PID: {} | Ring Entries: {}", res.0, res.1);
        }
    }
}

```

---

### C Comparison: Kernel-Space Only

The classic approach uses the `SEC(".maps")` syntax.

```c
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, u32);
    __type(value, u64);
} counters SEC(".maps");

SEC("kprobe/__x64_sys_io_uring_enter")
int bpf_trace_uring(struct pt_regs *ctx) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    u64 *count = bpf_map_lookup_elem(&counters, &pid);
    if (count) {
        __sync_fetch_and_add(count, 1);
    } else {
        u64 init = 1;
        bpf_map_update_elem(&counters, &pid, &init, BPF_ANY);
    }
    return 0;
}

```

### Comparison Summary

| Feature | Rust (Aya) | C (BPF) |
| --- | --- | --- |
| **Safety** | `Option`/`Result` prevents null derefs | Manual null-checks required |
| **Increment** | Direct dereference `*count += 1` | `__sync_fetch_and_add` (Atomic) |
| **Tooling** | `cargo xtask build` | `clang` + `Makefile` |


## Async Rust and networking:

For a networking and Rust developers, the most critical "Aha!" moment is realizing that 
**Standard Tokio** and **`tokio-uring`** have fundamentally different relationships with the Linux kernel.

Here is the step-by-step flow from the User Space (Rust/Tokio) down to the Kernel (Networking/IO).

---

### 1. Standard Tokio Flow (The "Readiness" Model)

This is the architecture used by 99% of Rust networking apps today. 

It is based on **Phase 2 (epoll)**.

* **User Space (Future Creation):**

    When you call `TcpStream::read().await`, 
    Rust creates a `Future`. 
    At this point, **no data has been requested from the kernel yet**.

* **The Reactor (Mio/epoll):** 
    
    Tokio registers your socket's file descriptor (FD) with `epoll` via the `mio` crate. 
    It tells the kernel: "Let me know when this socket has data ready."

* **The Wait:** 

    The executor puts the task to sleep. 
    One thread in your runtime is usually blocked in a `sys_epoll_wait` syscall, waiting for *any* of your
    thousands of connections to wake up.

* **The Wakeup:** 

    A packet arrives. 
    The NIC triggers an interrupt; the kernel marks the FD as "ready." 
    `epoll_wait` returns in user space.

* **The Actual I/O (The "Second" Step):** 

    Only *now* does Tokio wake up your task. 
    Your code then issues a **standard `read()` syscall**.

* **Networking vs. Disk:** 

    In this model, networking is truly async (non-blocking). 
    However, **Disk I/O is a "fake" async**: Since `epoll` doesn't support files well, Tokio silently
    spawns a thread from a background pool (`spawn_blocking`) to perform a standard, blocking `read()`.

---

### 2. `tokio-uring` Flow (The "Completion" Model)

This is the **Phase 3/4** approach. 

It is significantly faster for high-performance networking because it eliminates the "Second Step" 
described above.

* **User Space (Submission):** 

    When you call `tokio_uring::fs::File::read_at()`, you **must provide a buffer and give up ownership of
    it**.

* **The SQE (The Contract):** 
    
    `tokio-uring` writes a Submission Queue Entry (SQE) directly into the **shared memory ring**. 
    It doesn't ask the kernel "Is it ready?"; it says "Read this data into this specific memory address."

* **The Syscall (Optional):** 

    If `SQPOLL` is off, `tokio-uring` calls `sys_io_uring_enter` to tell the kernel there is new work. 
    If `SQPOLL` is on, **zero syscalls** happen—the kernel simply "sees" the new entry in the shared RAM.

* **Kernel Work:** 

    The kernel handles the I/O in the background (DMA from NIC/Disk directly to your buffer).

* **The CQE (The Result):** 

    When done, the kernel writes a Completion Queue Entry (CQE) to the second shared memory ring.

* **Rust Completion:** 

    The `Future` resolves. You get your buffer back, already filled with data. No second `read()` syscall 
    was ever needed.

---

### 3. Comparison Table (Networking Perspective)

| Feature | Standard Tokio (`mio`/`epoll`) | `tokio-uring` (`io_uring`) |
| --- | --- | --- |
| **Model** | **Readiness**: "Tell me when I can read." | **Completion**: "Tell me when you're done reading." |
| **Syscalls** | 1 `epoll_wait` + 1 `read` (per event) | 0 to 1 `io_uring_enter` (can batch 1000s) |
| **Ownership** | Borrowed: `&mut [u8]` | **Owned**: `Vec<u8>` (Required for safety) |
| **Zero-Copy** | Hard (requires specialized `splice`) | **Native**: Kernel writes directly to your Vec. |
| **Disk I/O** | Blocking (uses thread pool) | **True Async** |

---

### Summary for your eBPF Work:

If you are tracing a Rust program:

1. **If it's Standard Tokio:** You should trace the `syscalls:sys_enter_read` and `syscalls:sys_enter_epoll_wait`.
2. **If it's `tokio-uring`:** Those syscalls might not exist! You must trace the `io_uring` tracepoints or the `io_uring_enter` syscall.

## check if binary using `epoll` or `io_uring`:

**Using `bpftrace` command to check if a Rust binary is using the old `epoll` way or the new `io_uring` way**

Rust **Tokio** request moving from the high-level code down to the Linux kernel:

### 1. The Standard Tokio Path (Epoll-based)

This is the "Readiness" model used in 99% of current Rust networking.

* **User Space:** 

    You call `socket.read(&mut buf).await`. 
    Since the socket is empty, Tokio registers the FD and **yields**.

* **The Reactor:** 

    The `mio` crate (Tokio's backend) adds the FD to an **Interest List** via `epoll_ctl`.

* **The Kernel Sleep:** 

    The Tokio thread calls `epoll_wait` and blocks. The kernel monitors the NIC (Network Interface Card).

* **The Interrupt:** 

    A packet arrives. The NIC triggers an IRQ; the kernel marks the FD as "Ready."

* **The Wakeup:** 

    `epoll_wait` returns. Tokio wakes your task.

* **The Payload:**

    Your task resumes and performs a **standard `read()` syscall** to copy data from kernel space to your
    Rust `&mut [u8]`.

---

### 2. The `tokio-uring` Path (Proactor-based)

This is the "Completion" model (Phase 3/4).

* **User Space:** 

    You call `file.read_at(buf, offset).await`. 
    You **must** pass ownership of the buffer (`Vec<u8>`) to the kernel.

* **The Submission:** 

    `tokio-uring` writes an **SQE** (Submission Queue Entry) into the shared memory ring.

* **The Entry:** 

    It calls `io_uring_enter`. The kernel now "owns" your buffer memory.

* **The Background Work:** 

    The kernel performs the I/O (DMA) directly into your buffer while your Rust code continues other work.

* **The Completion:** 

    The kernel writes a **CQE** (Completion Queue Entry) into the second shared ring.

* **The Return:** 

    Tokio sees the CQE, resolves the Future, and hands the `Vec<u8>` back to you, already populated. 
    **No second `read()` syscall occurs.**

---

### 📋 Comparison Summary

| Feature | Standard Tokio | `tokio-uring` |
| --- | --- | --- |
| **Kernel Mechanism** | `epoll` | `io_uring` |
| **Buffer Handling** | Borrowed (`&mut [u8]`) | **Owned** (`Vec<u8>`) |
| **Efficiency** | 2 syscalls (wait + read) | **1 or 0 syscalls** (enter) |
| **Best For** | General microservices | **High-perf Networking / NVMe** |
