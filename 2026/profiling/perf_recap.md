# Linux Kernel Perf Subsystem — Detailed Explanation

## 1. What is Perf?

**Perf** (short for "Performance Events") is the Linux kernel's built-in profiling and tracing
infrastructure, introduced in kernel 2.6.31 (2009).

**Perf** provides a unified interface for accessing performance monitoring capabilities from:

- CPU **hardware performance counters** (PMU — Performance Monitoring Unit)
- **Software counters** implemented purely in the kernel
- **Tracepoints** (static instrumentation points in kernel code)
- **kprobes/uprobes** (dynamic instrumentation)
- **Hardware/software breakpoints**

Perf consists of two major parts:

1. **`perf_events` kernel subsystem** — the core infrastructure inside the kernel that manages events,
   counters, sampling, and data collection.

2. **`perf` userspace tool** — a command-line tool (`perf top`, `perf stat`, `perf record`, `perf report`, 
   etc.) that configures and reads data from the kernel subsystem via syscalls.

---

## 2. Architecture Overview

```
 ┌───────────────────────────────────────────────┐
 │            User-space (perf tool)             │ 
 │   perf stat / perf record / perf top / ...    │
 └───────────────┬───────────────────────────────┘
                 │ syscall: perf_event_open()
 ┌───────────────▼─────────────────────────────────┐
 │           perf_events core (kernel)             │
 │  - Event scheduling                             │
 │  - Context management (per-task/per-cpu)        │
 │  - Sampling & overflow handling                 │
 │  - Ring buffer (mmap) management                │
 └───────┬───────────────┬───────────────┬─────────┘
         │               │               │
 ┌───────▼──────┐ ┌──────▼───────┐ ┌─────▼─────────────┐
 │  PMU driver  │ │ Software     │ │ Tracepoints /     │
 │ (HW counters)│ │ counters     │ │ kprobes / uprobes │
 └──────────────┘ └──────────────┘ └───────────────────┘
```

### Key Kernel Data Structures

- **`struct perf_event`** — represents a single monitored event (e.g., "count L1 cache misses on CPU 2 for
  PID 1234").
- **`struct perf_event_context`** — a list of events grouped together, attached either to a task (thread) or
  a CPU.
- **`struct pmu`** — represents a Performance Monitoring Unit driver (could be hardware PMU, or a "fake" PMU
  for software events, tracepoints, breakpoints).
- **Ring buffer (`struct perf_buffer`)** — an mmap'd circular buffer shared between kernel and userspace
  used to deliver sampled data without constant syscalls.

### Core Syscall: `perf_event_open()`

```c
int perf_event_open(struct perf_event_attr *attr, pid_t pid,
                     int cpu, int group_fd, unsigned long flags);
```

This single syscall is the entry point to the entire subsystem. The `perf_event_attr` structure specifies:
- **type** (HARDWARE, SOFTWARE, TRACEPOINT, HW_CACHE, RAW, BREAKPOINT, ...)
- **config** (which specific event, e.g., `PERF_COUNT_HW_CPU_CYCLES`)
- Sampling period/frequency
- Flags: inherit to children, exclude kernel/user, pinned, exclusive, etc.

The returned file descriptor can be:
- **read()** for simple counting
- **mmap()**'d for a ring buffer of samples
- **ioctl()**'d to enable/disable/reset the event
- Used as `group_fd` to group multiple events together (so they're scheduled on the PMU simultaneously)

---

## 3. Two Modes of Operation

### a) Counting Mode
The event just accumulates a count over time (e.g., total number of instructions executed). Read via
`read()`. Used by `perf stat`.

### b) Sampling Mode
The event triggers an interrupt after N occurrences (or every N nanoseconds in frequency mode), and at each
trigger, the kernel records a **sample** (IP, PID, TID, callchain, registers, etc.) into the mmap'd ring
buffer. Used by `perf record`.

---

## 4. Per-Task vs Per-CPU Contexts

Events can be attached to:
- **A specific task** — counts only while that task is running (context-switched in/out with the task)
- **A specific CPU** — counts everything running on that CPU regardless of task
- **Task + CPU combination**
- **System-wide** — all CPUs, requires elevated privileges

The kernel handles **event multiplexing**: if there are more events requested than physical hardware
counters available, the kernel time-slices the events across the counters and scales the final count based
on `time_enabled` / `time_running` ratios.

---

## 5. Now — Part 2: Hardware and Software Units perf_events Interacts With

`perf_events` is designed around a generic `struct pmu` abstraction, so **any countable/traceable source**
in the kernel can be exposed to it. These fall into several categories:

### A. Hardware PMU (Performance Monitoring Unit) Events — `PERF_TYPE_HARDWARE` / `PERF_TYPE_RAW`

These are counters implemented in **silicon** (CPU hardware) that count microarchitectural events. Perf
talks to the CPU's PMU driver (e.g., Intel's `perf_event_intel`, AMD's `perf_event_amd`, ARM's PMUv3
driver).

**Generalized hardware events** (portable across architectures, `PERF_TYPE_HARDWARE`):
- `PERF_COUNT_HW_CPU_CYCLES` — CPU clock cycles
- `PERF_COUNT_HW_INSTRUCTIONS` — retired instructions
- `PERF_COUNT_HW_CACHE_REFERENCES` / `PERF_COUNT_HW_CACHE_MISSES` — last-level cache accesses/misses
- `PERF_COUNT_HW_BRANCH_INSTRUCTIONS` / `PERF_COUNT_HW_BRANCH_MISSES` — branch prediction stats
- `PERF_COUNT_HW_BUS_CYCLES`
- `PERF_COUNT_HW_STALLED_CYCLES_FRONTEND` / `_BACKEND` — pipeline stall cycles
- `PERF_COUNT_HW_REF_CPU_CYCLES` — reference (non-scaled) cycles

**Raw/model-specific events** (`PERF_TYPE_RAW`): Direct access to vendor-specific PMU event codes
(documented in Intel SDM Vol. 3B or AMD PPRs), e.g. cache-line contention, TLB walks, uop dispatch stalls.
Used when generic events aren't granular enough.

**HW_CACHE events** (`PERF_TYPE_HW_CACHE`): A matrix of {cache level (L1D/L1I/LL/DTLB/ITLB/BPU) × operation
(read/write/prefetch) × result (access/miss)}.

**How it interacts:** The PMU driver programs **Model-Specific Registers (MSRs)** on x86 (or equivalent
config registers on ARM/PowerPC) to select the event and set up the counter. When the counter overflows (in
sampling mode), it raises a **Performance Monitoring Interrupt (PMI/NMI)**, which perf's interrupt handler
catches to record a sample. There's a limited number of physical counters (typically 4–8 general-purpose + a
few fixed-function ones per core), which is why multiplexing exists.

### B. Software Events — `PERF_TYPE_SOFTWARE`

These are **counted entirely in the kernel**, not by hardware — they hook into kernel code paths directly.

- `PERF_COUNT_SW_CPU_CLOCK` — wall-clock time the CPU ran (kernel high-res timer based)
- `PERF_COUNT_SW_TASK_CLOCK` — time the specific task was scheduled on a CPU
- `PERF_COUNT_SW_PAGE_FAULTS` (+ `_MIN` / `_MAJ` variants) — page fault counts, hooked into the fault handler
- `PERF_COUNT_SW_CONTEXT_SWITCHES` — hooked into the scheduler's `context_switch()`
- `PERF_COUNT_SW_CPU_MIGRATIONS` — task moved between CPUs
- `PERF_COUNT_SW_ALIGNMENT_FAULTS`, `PERF_COUNT_SW_EMULATION_FAULTS`
- `PERF_COUNT_SW_DUMMY` — used for group leader placeholders
- `PERF_COUNT_SW_BPF_OUTPUT` — used with eBPF programs writing directly into perf ring buffers

**How it interacts:** These don't need a physical counter — they're just increments (`perf_sw_event()`)
called from the relevant kernel subsystem (scheduler, mm/fault handler, etc.) whenever the corresponding
event occurs. This makes them available even on hardware without a PMU, or in VMs where hardware PMU access
may be restricted.

### C. Tracepoints — `PERF_TYPE_TRACEPOINT`

Static, compiled-in instrumentation points placed by kernel developers at meaningful locations (`TRACE_EVENT()` macro), e.g.:
- `sched:sched_switch`, `sched:sched_wakeup`
- `syscalls:sys_enter_*` / `sys_exit_*`
- `block:block_rq_issue`
- `kmem:kmalloc`, `kmem:kfree`
- `irq:irq_handler_entry`

**How it interacts:** Tracepoints are backed by the **ftrace** infrastructure. perf_events registers a probe
callback with the tracepoint; when the code path executes `trace_<name>()`, all registered callbacks
(including perf's) fire, and perf records the event (with structured fields) into the ring buffer. Very low
overhead when disabled (uses static jump patching / `jump_label`).

### D. Dynamic Probes — kprobes / uprobes — `PERF_TYPE_TRACEPOINT` (dynamically created)

- **kprobes**: Dynamically insert a breakpoint at (almost) any kernel instruction address at runtime,
  without needing a precompiled tracepoint. Implemented by replacing the instruction with a trap (e.g.,
  `int3` on x86) and single-stepping.
- **uprobes**: Same concept but for **userspace** binaries/libraries — lets perf trace arbitrary user-space
  function entry/exit without modifying the binary or recompiling.

**How it interacts:** These create a temporary/dynamic tracepoint-like event registered with
`perf_event_open()` using `PERF_TYPE_TRACEPOINT` after being defined via
`/sys/kernel/debug/tracing/kprobe_events` (or via `perf probe`).

### E. Hardware Breakpoints — `PERF_TYPE_BREAKPOINT`

Uses the CPU's **debug registers** (e.g., x86 `DR0–DR3`) to trap on memory access (read/write/execute) to a
specific address, independent of software instrumentation. Useful for watching a variable or catching
illegal memory access.

### F. Uncore / Off-core PMUs

On modern multi-core systems, in addition to per-core PMUs, there are **uncore PMUs** monitoring shared
resources: the last-level cache, memory controller, QPI/UPI interconnect links, integrated I/O. These appear
as separate PMU instances (e.g., `uncore_imc_0`) exposed under `/sys/bus/event_source/devices/`, and perf
can address them explicitly (`perf stat -e uncore_imc_0/event=...`).

### G. Other Specialized PMU Sources
- **Power events** (RAPL — Running Average Power Limit — on Intel, exposed as `power/energy-cores/`, etc.)
- **PEBS (Precise Event-Based Sampling)** on Intel — hardware-assisted precise sampling that avoids "skid"
  (the delay between event occurrence and interrupt) by having hardware write sample data directly to a
  buffer.
- **Intel PT (Processor Trace)** / **ARM CoreSight** — full instruction-flow tracing integrated via perf's
  `aux` ring buffer mechanism.
- **BTS (Branch Trace Store)** — hardware log of branch history.

---

## 6. How These Are Exposed to Userspace

- `/sys/bus/event_source/devices/` — lists all registered PMUs on the system (core PMU, uncore PMUs,
  breakpoint, software, tracepoint, etc.)
- `/sys/kernel/debug/tracing/events/` — lists all available tracepoints
- `perf list` — userspace command showing all available events across all these categories

---

## Summary Table

| Category | PERF_TYPE | Backed By | Example |
|---|---|---|---|
| Hardware generic | `PERF_TYPE_HARDWARE` | CPU PMU counters | `cycles`, `instructions` |
| Hardware raw | `PERF_TYPE_RAW` | CPU PMU (vendor-specific MSRs) | `r534a` (raw event code) |
| Cache matrix | `PERF_TYPE_HW_CACHE` | CPU PMU | L1D-load-misses |
| Software | `PERF_TYPE_SOFTWARE` | Kernel code hooks | `page-faults`, `context-switches` |
| Tracepoint | `PERF_TYPE_TRACEPOINT` | ftrace static probes | `sched:sched_switch` |
| Dynamic probe | `PERF_TYPE_TRACEPOINT` (dynamic) | kprobes/uprobes | custom function entry |
| Breakpoint | `PERF_TYPE_BREAKPOINT` | CPU debug registers | watch memory address |
| Uncore | custom PMU | Shared silicon (memory ctrl, interconnect) | `uncore_imc_0/...` |

If you'd like, I can also walk through a concrete example (e.g., tracing `perf record -e cache-misses -c
100000 ./app` end-to-end through the kernel) or dive into the ring-buffer/mmap sampling mechanism in more
depth.

--- 

# Hardware & Software Counters — Kernel-Level View Across Architectures

## 1. The Core Abstraction: `struct pmu`

Regardless of architecture, the kernel treats **every** countable source — real silicon counters or fake software counters — as a `struct pmu` instance. This is the key design decision that makes perf portable.

```c
struct pmu {
    struct list_head    entry;
    struct device        *dev;
    const struct attribute_group **attr_groups;
    const char           *name;
    int                  type;          /* PERF_TYPE_* or dynamically assigned */

    int (*event_init)   (struct perf_event *event);
    void (*add)          (struct perf_event *event, int flags);
    void (*del)          (struct perf_event *event, int flags);
    void (*start)        (struct perf_event *event, int flags);
    void (*stop)         (struct perf_event *event, int flags);
    void (*read)         (struct perf_event *event);
    ...
};
```

Every architecture (or subsystem) that wants to expose countable events **registers its own `pmu`
implementation** via `perf_pmu_register()`. So there isn't one "the PMU driver" — there's the x86 core PMU
driver, the AMD IBS driver, uncore PMU drivers, the software PMU, the tracepoint PMU, the breakpoint PMU,
etc., all coexisting and each visible under `/sys/bus/event_source/devices/<pmu-name>/`.

This is *why* `perf stat -e cycles` works identically whether you're on x86, ARM, or POWER — the generic
`perf_event_attr.type = PERF_TYPE_HARDWARE, config = PERF_COUNT_HW_CPU_CYCLES` gets translated by **each
arch's own mapping table** into whatever the real silicon needs.

---

## 2. Software Counters — Architecture Independent

Software counters live entirely in `kernel/events/core.c` (the generic `perf_swevent` code) and require **no
arch-specific code at all**. They work identically on x86_64, ARM64, RISC-V, PowerPC, MIPS, or even
architectures with no PMU support whatsoever.

Implementation mechanics:
- A `swevent_hlist` (per-CPU hash table) tracks active software events, keyed by event type.
- Kernel subsystems call a generic hook — e.g. `perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, addr)` —
  directly from `handle_mm_fault()`, `context_switch()`, timer interrupts, etc.
- No counter register is programmed; it's just an atomic increment plus overflow/sampling-period check done
  in software.
- `PERF_COUNT_SW_CPU_CLOCK` / `TASK_CLOCK` are driven by **hrtimers**, not by any hardware cycle counter —
  they use `local_clock()` (which itself may be backed by TSC on x86 or a generic arch counter elsewhere,
  but that's abstracted away).

Because they need no hardware support, software events are the fallback used in restricted environments —
nested VMs, containers without host PMU passthrough, or architectures the perf hardware backend hasn't been
ported to.

---

## 3. Hardware Counters on x86_64

### a) Core PMU (per-logical-CPU)

x86 hardware counters are implemented via **Model-Specific Registers (MSRs)**, accessed with the
`RDMSR`/`WRMSR` instructions (kernel-only, CPL0). The registration path:

- Driver location: `arch/x86/events/` (`core.c`, `intel/core.c`, `amd/core.c`)
- On boot, `init_hw_perf_events()` detects the CPU vendor/family and calls into `intel_pmu_init()` or
  `amd_pmu_init()`, which populates a `struct x86_pmu` describing:
  - Number of general-purpose counters (`IA32_PMCx`) — typically 4 per core (8 with hyperthreading
    disabled/some server parts)
  - Number of fixed-function counters (`IA32_FIXED_CTRx`) — usually 3: instructions retired, unhalted core
    cycles, unhalted ref cycles
  - Counter width (typically 48 bits, some newer CPUs 57 bits)
  - Event-to-MSR mapping table (translates generic `PERF_COUNT_HW_*` into vendor event-select codes)

**Key MSRs involved:**
| MSR | Purpose |
|---|---|
| `IA32_PERFEVTSELx` | Selects event code + umask + enable/user/os/int bits for counter x |
| `IA32_PMCx` | The actual counter value |
| `IA32_PERF_GLOBAL_CTRL` | Master enable/disable for all counters at once |
| `IA32_PERF_GLOBAL_STATUS` | Overflow status bits, read at PMI time |
| `IA32_PERF_GLOBAL_OVF_CTRL` | Clears overflow bits |
| `IA32_FIXED_CTR_CTRL` | Controls the 3 fixed-function counters |
| `IA32_DEBUGCTLMSR` | Enables LBR (Last Branch Record), BTS |

Because these are **per-logical-CPU MSRs**, and the kernel schedules `perf_event`s onto counters using
`x86_pmu.enable()`/`.disable()` inside IRQ-disabled sections, the "add" callback for the x86 core PMU
literally writes to these registers on the physical core the task is running on.

**Overflow delivery:** When a counter overflows, the CPU raises the **Local APIC Performance Monitoring
Interrupt (LVT PMI entry)**, typically routed as an **NMI** on modern CPUs (so that PMI can even fire inside
regions where regular interrupts are masked, e.g. inside `local_irq_disable()`). This lands in
`perf_event_nmi_handler()` → walks `IA32_PERF_GLOBAL_STATUS`, finds which counter overflowed, records a
sample, resets the counter, clears the status bit via `GLOBAL_OVF_CTRL`.

**Multiplexing:** Since there are only ~4–8 general counters but potentially many requested events,
`perf_rotate_context()` runs periodically (default every 1ms tick) to round-robin which events actually
occupy hardware counters, scaling reported counts by `time_running/time_enabled`.

### b) Precise sampling — PEBS

Regular sampling has "skid" — by the time the PMI fires and the kernel reads `RIP`, several instructions may
have already executed past the actual overflow point (out-of-order execution). **PEBS (Precise Event-Based
Sampling)** solves this: the hardware itself, on the *exact* micro-op that caused overflow, DMAs a
fixed-format record (registers, RIP, data address, latency) directly into a buffer in memory (the "PEBS
buffer", configured via `IA32_PEBS_ENABLE`/`DS_AREA` MSR) — bypassing interrupt latency entirely. perf
surfaces this as `event/pebs=1` or automatically for certain "precise" events (`cycles:pp` etc.).

### c) Uncore PMUs

Shared resources (last-level cache slices, integrated memory controller, QPI/UPI links, IIO) have **separate
counter blocks**, physically outside the core, addressed via PCI config space or MSRs specific to the uncore
unit — not tied to any particular logical CPU. Kernel drivers: `arch/x86/events/intel/uncore.c`. Each shows
up as its own `pmu` (`uncore_imc_0`, `uncore_qpi_0`, etc.) under sysfs, since these counters are shared
across cores and don't belong to any one task/thread context.

### d) AMD IBS (Instruction-Based Sampling)

AMD's alternative/complement to PEBS — samples fetched or executed instructions with rich metadata (cache
miss latency, branch info) via `MSRC001_1030+` registers. Exposed as `ibs_fetch`/`ibs_op` PMUs.

### e) Intel PT / LBR

- **LBR (Last Branch Record)**: a small hardware ring buffer of recent branch source/target pairs, read via
  `MSR_LASTBRANCH_x_FROM_IP`/`TO_IP`.
- **Intel PT (Processor Trace)**: full control-flow trace written to a dedicated memory buffer, exposed
  through perf's separate **AUX ring buffer** (distinct from the normal sample ring buffer), since the data
  volume/format is fundamentally different from regular samples.

---

## 4. Hardware Counters on Other Architectures

### ARM64 (AArch64)

- Driver: `drivers/perf/arm_pmu.c` + `arch/arm64/kernel/perf_event.c`, implementing **PMUv3** (the ARM
  architected PMU, part of the AArch64 architecture spec, not vendor-specific like x86 MSRs).
- Accessed via **system registers** (not MSRs) using `MRS`/`MSR` instructions: `PMEVCNTRn_EL0` (counters),
  `PMEVTYPERn_EL0` (event select), `PMCR_EL0` (control), `PMOVSCLR_EL0` (overflow status), `PMINTENSET_EL1`
  (interrupt enable).
- Typically **6 programmable counters + 1 fixed cycle counter**, though this varies by implementation.
- Interrupt delivery is a normal **PPI (Private Peripheral Interrupt)** via the GIC, not an NMI-equivalent
  (though newer ARMv8.4+ parts support a true NMI class for this).
- ARM's architected event numbers (e.g., `0x08` = instructions retired) are standardized in the ARM ARM
  (Architecture Reference Manual), which is why generic events map fairly cleanly — but vendors (Apple,
  Qualcomm, Ampere, etc.) also add proprietary raw events for their microarchitecture.
- Because ARM SoCs are heterogeneous (big.LITTLE / DynamIQ), the kernel may register **multiple core PMU
  instances** — one per CPU cluster type — since different core types have different counters/events. `perf`
  shows these as e.g. `armv8_cortex_a76`, `armv8_cortex_a55`.
- Uncore equivalents exist too, via **CoreSight** and vendor-specific system PMUs (e.g., DDR controller PMUs
  on server SoCs), each a distinct `pmu` device.
- **ARM CoreSight** = the ARM analog to Intel PT for full instruction trace, integrated via the same AUX
  buffer mechanism in perf.

### PowerPC (Power8/9/10)

- Driver: `arch/powerpc/perf/`
- Uses **Monitor Mode Control Registers (MMCRx)** and Performance Monitor Counters (`PMC1`–`PMC6`), accessed
  via `mtspr`/`mfspr` (move to/from special-purpose register) instructions.
- Has a distinctive feature: counters can trigger a **Performance Monitor exception**, and Power ISA also
  supports **marked events** somewhat analogous to PEBS for precise sampling.

### RISC-V

- Newer and less mature; based on the **Sscofpmf**/**Zicntr** extensions and the SBI PMU extension for
  counter access, since RISC-V's base ISA historically only mandated a cycle/instret counter with everything
  else vendor-defined (unlike ARM's architected PMUv3). Driver: `drivers/perf/riscv_pmu_sbi.c`.

### s390x, MIPS, SPARC

Each has its own arch-specific driver under `arch/<arch>/kernel/perf_event.c` or `drivers/perf/`, following
the same `struct pmu` contract but talking to completely different hardware counter register sets.

---

## 5. How the Kernel Unifies This: Generic Event Translation

The key portability trick is in each arch's PMU driver providing a table like this (simplified, x86 Intel
example):

```c static const u64 intel_perfmon_event_map[] = { [PERF_COUNT_HW_CPU_CYCLES]    = 0x003c,
[PERF_COUNT_HW_INSTRUCTIONS]  = 0x00c0, [PERF_COUNT_HW_CACHE_REFERENCES] = 0x4f2e,
[PERF_COUNT_HW_CACHE_MISSES]  = 0x412e, [PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = 0x00c4,
[PERF_COUNT_HW_BRANCH_MISSES] = 0x00c5, }; ```

ARM's equivalent maps the same generic enum to its architected event numbers (e.g.
`PERF_COUNT_HW_INSTRUCTIONS → 0x08`). This table lookup happens inside each PMU's `.event_init()` callback,
which is invoked generically by `perf_event_open()` regardless of arch — the syscall layer, `perf_event`
struct, ring buffer, sampling logic, and userspace ABI are **100% architecture-independent**; only this
translation layer plus the actual register programming differs per arch.

---

## 6. Visibility from Userspace (any arch)

```bash # List all registered PMUs on this system ls /sys/bus/event_source/devices/ # e.g. on x86_64 server:
cpu, uncore_imc_0, uncore_qpi_0, breakpoint, software, tracepoint, msr... # e.g. on ARM64 big.LITTLE:
armv8_cortex_a76, armv8_cortex_a55, breakpoint, software, tracepoint

# Inspect what raw event codes a symbolic name maps to on THIS arch cat
/sys/bus/event_source/devices/cpu/events/instructions # -> event=0xc0   (x86 example)

# Full enumeration, includes vendor/arch-specific extras perf list ```

---

## Summary Comparison Table

| Aspect | x86_64 | ARM64 | PowerPC |
|---|---|---|---|
| Register access | `RDMSR`/`WRMSR` | `MRS`/`MSR` sys regs | `mfspr`/`mtspr` |
| Typical counter count | 4–8 GP + 3 fixed | 6 + 1 fixed (varies) | up to 6 (`PMC1-6`) |
| Overflow interrupt | NMI (LVT PMI) | PPI via GIC | Perf Monitor exception |
| Precise sampling | PEBS | (SPE - Statistical Profiling Ext. on some cores) | Marked events |
| Full trace | Intel PT | CoreSight | — |
| Event standardization | Vendor-specific (Intel/AMD differ) | Architected (PMUv3 spec) + vendor raw | Vendor-specific (IBM) |
| Uncore/shared units | Separate PCI/MSR uncore PMUs | Separate SoC/CoreSight PMUs | Nest/core-level MMUs |

The elegance of the design is that **software events, tracepoints, and the syscall/ring-buffer ABI never change across this table** — only the shaded region of "how do I program and read a hardware counter" is arch-specific, hidden entirely behind the `struct pmu` interface.



--- 
# Additional Info:

## 1. PMU 
## 2. PMU ( From  systems programming view )

--- 

# 1. PMU : 

A **CPU hardware performance counter** is a small hardware register inside the CPU that automatically 
counts specific low-level events while a program is running. 

These counters are managed by the **Performance Monitoring Unit (PMU)**, a dedicated subsystem built into 
modern processors.

Think of the PMU as the CPU's built-in telemetry system. 
Instead of guessing what your program is doing, it measures what actually happened in hardware.

### What can the PMU count?

Depending on the processor, it can count events such as:

| Event                 | What it tells you                                    |
| --------------------- | ---------------------------------------------------- |
| CPU cycles            | How long execution took in terms of processor cycles |
| Instructions retired  | Number of completed instructions                     |
| Cache hits/misses     | How effectively the cache is being used              |
| Branch instructions   | Number of branch operations                          |
| Branch mispredictions | How often the CPU guessed a branch incorrectly       |
| Memory loads/stores   | Memory access activity                               |
| TLB misses            | Failures in address translation cache                |
| Pipeline stalls       | When execution had to wait for data or resources     |

For example, suppose a program executes:

```c
for (int i = 0; i < N; i++)
    sum += A[i];
```

The PMU might report:

* 2.3 billion CPU cycles
* 1.1 billion instructions retired
* 12 million L1 cache misses
* 250,000 branch mispredictions

This gives you much deeper insight than simply knowing the program took 0.8 seconds.

---

## Why are hardware counters useful?

Performance engineers use them to answer questions like:

* Why is my program slow?
* Is it waiting on memory?
* Is the cache too small?
* Are branches being mispredicted?
* Is vectorization actually helping?
* Is the CPU fully utilized?

Instead of saying:

> "The program is slow."

you can say:

> "40% of execution time is lost due to LLC cache misses."

---

## Why are they called "hardware" counters?

Because the counting happens inside the processor itself.

Unlike software profiling, the PMU increments counters in hardware as instructions execute.

For example:

```
Instruction executes
        │
        ▼
CPU hardware
        │
        ▼
PMU increments:
    instructions_retired++
```

No instrumentation is needed in your program.

---

## Example counters

A processor might expose counters like:

```
CPU cycles
Instructions retired
L1 cache misses
L2 cache misses
LLC cache misses
Branch instructions
Branch misses
Floating-point operations
SIMD instructions
Memory accesses
```

Modern CPUs often support hundreds of event types, although only a limited number can usually be measured
simultaneously.

---

## Example on Linux

The `perf` tool reads PMU counters.

```bash
perf stat ./my_program
```

Output might look like:

```
1,025,134,782 cycles
2,841,510,226 instructions
12,391 cache-misses
1,203 branch-misses
```

Or profile where time is spent:

```bash
perf record ./my_program
perf report
```

---

## Common performance metrics derived from counters

The raw counts are often combined into more meaningful metrics:

* **IPC (Instructions Per Cycle)**

  ```
  IPC = instructions / cycles
  ```

  Higher IPC generally means the CPU is executing more useful work each cycle.

* **Cache miss rate**

  ```
  misses / accesses
  ```

* **Branch prediction accuracy**

  ```
  1 - (branch_misses / branches)
  ```

These metrics help identify bottlenecks.

---

## How the PMU works

A simplified view:

```
           Program
              │
              ▼
      CPU executes instructions
              │
              ▼
       Performance Monitoring Unit
              │
   ┌──────────┼──────────┐
   ▼          ▼          ▼
Cycles     Cache Miss   Branch Miss
Counter      Counter      Counter
```

As the CPU runs, the PMU updates these counters automatically.

---

## Sampling vs. counting

The PMU supports two common modes:

1. **Counting mode**

   * Reports total event counts after execution.
   * Example:

     * 5 billion cycles
     * 10 million cache misses

2. **Sampling mode**

   * Generates an interrupt after a counter reaches a threshold.
   * Records which instruction was executing at that moment.
   * Lets profilers identify *where* expensive events occur, not just *how many*.

Tools like `perf record` rely on sampling mode to build performance profiles.

---

## Limitations

PMUs are powerful but have some constraints:

* CPUs typically have only a small number of physical counters (often 4–8 general-purpose counters), so not
  every event can be measured simultaneously.
* Available events differ across processor families and models.
* Some events are only estimates or require careful interpretation.
* Measuring performance introduces a small amount of overhead, especially in sampling mode.

---

### In summary

A **Performance Monitoring Unit (PMU)** is dedicated hardware inside the CPU that counts low-level execution
events such as cycles, instructions, cache misses, and branch mispredictions. Hardware performance counters
provide precise measurements of how software interacts with the processor, making them an essential tool for
profiling, performance tuning, compiler optimization, and computer architecture research.

---
#  2. PMU  ( from systems programming view: )

Hardware performance counters are especially valuable in **systems programming** because you're often trying
to understand how software interacts with the CPU, memory hierarchy, and operating system. They're somewhat
less central in **network programming**, but they become very useful when optimizing high-throughput or
low-latency systems.

## Systems programming

Systems programmers work on kernels, runtimes, databases, memory allocators, filesystems, and other
low-level software where hardware behavior has a major impact on performance.

### 1. Finding cache problems

Suppose you write a custom memory allocator:

```text malloc() ↓ returns scattered memory ↓ CPU cache misses increase ↓ program slows down ```

Without PMU counters, you only know the program is slower.

With PMU counters, you might see:

* L1 cache misses: normal
* L2 cache misses: slightly higher
* LLC (Last-Level Cache) misses: 5× higher

This points to poor memory locality rather than inefficient algorithms.

---

### 2. Detecting branch prediction issues

Consider this code:

```c if (random_value > threshold) do_A(); else do_B(); ```

If the condition is unpredictable, the CPU's branch predictor often guesses wrong, forcing the processor to
discard speculative work.

A high **branch-misprediction** count suggests rewriting the code or using branchless techniques could
improve performance.

---

### 3. Measuring instruction efficiency

Imagine two implementations of the same function:

Version A:

* 3 billion instructions
* 2 billion cycles

Version B:

* 2 billion instructions
* 2 billion cycles

Even if both take similar time, Version B accomplishes the same work with fewer instructions, which may
reduce power consumption or free CPU resources.

---

### 4. Investigating memory stalls

Modern CPUs can execute several instructions simultaneously, but they stall when waiting for data from
memory.

PMU counters can reveal:

* Front-end stalls (instruction fetch/decode bottlenecks)
* Back-end stalls (execution units waiting on memory or other resources)

This helps determine whether your optimization effort should focus on computation or memory access.

---

### 5. Comparing data structures

Suppose you're choosing between:

* Linked list
* Dynamic array
* Hash table

Their algorithmic complexity doesn't tell the whole story. PMU counters can show that a linked list causes
many more cache misses because each node may be in a different memory location, while an array benefits from
contiguous memory.

---

## Network programming

Network applications are often limited by the network itself, but in high-performance networking the CPU
becomes a bottleneck. PMU counters help identify where that bottleneck lies.

### 1. Packet processing

Suppose you're writing a packet-processing application:

```text NIC ↓ Kernel ↓ Your packet parser ↓ Forward packet ```

If you can process only 4 million packets per second instead of 10 million, PMU counters can indicate
whether you're spending time:

* Waiting for memory
* Mis-predicting branches
* Executing too many instructions
* Stalling in the CPU pipeline

---

### 2. Optimizing packet parsing

Consider:

```c switch(packet_type) { ... } ```

If packet types arrive in a random order, branch prediction may perform poorly.

PMU counters showing many branch mispredictions could motivate a different parsing strategy, such as lookup
tables or SIMD-based parsing.

---

### 3. Improving cache locality

High-speed servers often maintain structures like:

```text Connection table Routing table Session cache TCP state ```

If these structures don't fit well in the CPU cache, packet processing slows because of frequent memory
accesses.

PMU counters can reveal high cache-miss rates, suggesting that reorganizing data layouts or reducing
per-connection state may help.

---

### 4. Evaluating zero-copy networking

Zero-copy techniques avoid unnecessary copying of packet data between buffers.

Without PMU data:

> "It feels faster."

With PMU data:

* 30% fewer instructions
* 40% fewer memory loads
* Higher instructions per cycle (IPC)
* Lower cache-miss rate

This provides concrete evidence of the improvement.

---

## Real-world examples

Many high-performance systems rely on PMU data during development and tuning:

* **Operating systems** measure scheduler and memory-management behavior.
* **Databases** optimize cache efficiency and query execution.
* **Web servers** reduce latency by improving memory access patterns.
* **Network packet-processing frameworks** optimize per-packet CPU costs.
* **Language runtimes** and **garbage collectors** use PMU data to refine memory management and execution
  strategies.

---

## Example: Optimizing a network server

Imagine a server handling 1 million requests per second.

Initial measurements:

| Metric          |          Value |
| --------------- | -------------: |
| CPU utilization |           100% |
| Throughput      | 1 M requests/s |
| Cache misses    |      Very high |
| Branch misses   |       Moderate |

After reorganizing frequently accessed data to improve locality:

| Metric                 | Before |            After |
| ---------------------- | -----: | ---------------: |
| L1 cache misses        |   12 M |              4 M |
| LLC cache misses       |    3 M |            800 K |
| Instructions per cycle |    0.9 |              1.8 |
| Throughput             |    1 M | 1.8 M requests/s |

The algorithm didn't change—the improvement came from making the code more CPU-friendly.

## Should you learn PMU counters?

If your goal is to become a systems programmer, especially in areas like operating systems, databases,
high-performance networking, or storage systems, understanding PMU counters is a valuable skill. They let
you move beyond "this code is slow" to identifying *why* it's slow at the hardware level. For everyday
application or typical web development, you may rarely need them, but for performance-critical systems
software, they're among the most powerful diagnostic tools available.
