# The Linux Kernel eBPF Execution Model

---
## High level overview:

### **1. What is eBPF?**

**eBPF (extended Berkeley Packet Filter)** is a virtual machine inside the Linux kernel that allows safe, 
efficient execution of user-defined programs inside the kernel. 

Originally designed for packet filtering, it now supports:
* Networking (XDP, TC)
* Security (seccomp, LSM)
* Observability (tracing, performance profiling)

Key point: eBPF lets you **run code in the kernel safely**, without writing kernel modules.

---

### **2. eBPF Execution Model Overview**

At a high level, the **execution model** describes how eBPF programs are loaded, verified, and executed in 
the kernel. 

It consists of several stages:

#### **Step 1: Program Loading**

* Userspace compiles a program (usually in C) to **eBPF bytecode**.
* The bytecode is loaded into the kernel using the `bpf()` system call.
* Example:

  ```c
  int prog(struct __sk_buff *skb) {
      return 0; // trivial program
  }
  ```

#### **Step 2: Verification**

Before running anything in kernel space, **the verifier** checks the program:

* Ensures **safety**: no out-of-bounds memory access, no loops (or bounded loops), proper use of maps.
* Ensures **termination**: program cannot hang the kernel.
* Enforces **type safety** for kernel structures.

If the program fails verification, it is **rejected**.

**Why it matters:** The verifier makes eBPF programs safe to execute without crashing the kernel.

---

#### **Step 3: Just-In-Time (JIT) Compilation**

* Once verified, eBPF bytecode can be:

  * **Interpreted:** executed instruction-by-instruction (slower)
  * **JIT compiled:** converted into native machine code for faster execution
* This step happens automatically in many kernels if JIT is enabled.

---

#### **Step 4: Execution Context**

eBPF programs are executed **in a restricted context** depending on the hook point:

* **XDP programs:** run at the earliest point in the network stack, for packet processing.
* **tc/BPF programs:** run in traffic control (ingress/egress) hooks.
* **Tracepoints / kprobes / uprobes:** run on kernel function entry/exit or user function entry/exit.
* **cgroups / LSM hooks:** enforce security policies.

Key constraints during execution:

* No loops (unless bounded)
* No arbitrary memory access
* Must only call **allowed helper functions**
* Runs **synchronously**, in the kernel context where the hook is triggered

---

#### **Step 5: Accessing Data**

eBPF programs interact with the system via:

1. **Maps** (key-value stores in kernel memory)

   * Shared between user-space and kernel-space
   * Example: `BPF_HASH`, `BPF_ARRAY`, `BPF_PERCPU_ARRAY`
2. **Helper functions**

   * Predefined kernel functions, e.g., `bpf_map_lookup_elem()`, `bpf_trace_printk()`
3. **Context data**

   * Kernel passes a **context pointer** depending on the hook
   * Example: `struct __sk_buff *skb` for packet data

---

### **3. eBPF Instruction Model**

* **Registers:** eBPF has 10 general-purpose 64-bit registers (R0–R9).

  * R0: return value
  * R1–R5: arguments to helpers
  * R6–R9: callee-saved registers
* **Stack:** 512 bytes of per-program stack
* **Instructions:** 64-bit, similar to RISC instructions

  * Arithmetic, logical, memory load/store, calls to helpers
* **No direct kernel memory access:** everything goes through helpers or maps.

---

### **4. Lifecycle Summary**

Here’s the **flow of an eBPF program**:

1. Userspace writes C → compiles to eBPF bytecode
2. Load bytecode → `bpf()` syscall
3. Kernel verifier checks safety and correctness
4. Optional JIT compilation → native code
5. Program is attached to a hook (XDP, tracepoint, cgroup, etc.)
6. Program executes whenever the hook is triggered, interacting with context, maps, and helpers

---

### **5. Key Features of the Execution Model**

* **Safety first:** verifier ensures no unsafe memory or CPU use
* **High performance:** optional JIT allows near-native speed
* **Context-specific:** execution model depends on hook type
* **Observability and interactivity:** maps allow communication with user-space
* **Extensibility:** supports custom programs without kernel recompilation

---

**Analogy:** Think of eBPF as a sandboxed mini-kernel inside the kernel. You give it a small, safe, 
verified program, and it executes at high speed whenever a certain kernel event happens.

---

## How kernel invokes an eBPF program and passes data to it:


Data passing mechanism, including contexts, registers, and helper calls:

---

### **1. Kernel Hook Invocation**

eBPF programs are **attached to hooks** in the kernel.
These hooks determine **when the program runs** and what **data it can access**. 

Examples include:

* **Network hooks:** XDP, tc (ingress/egress)
* **Tracepoints/kprobes:** function entry/exit points
* **cgroup hooks:** process/network policies
* **LSM hooks:** security enforcement

When the kernel reaches a hook:

1. It **checks if an eBPF program is attached**.
2. If yes, it calls a **common eBPF trampoline** in the kernel.

---

### **2. Passing Data: The Context Pointer**

The kernel passes data to the eBPF program via a **context structure**, which is **hook-specific**:

* **XDP programs:** get `struct xdp_md *ctx`

  * Contains packet data, metadata, and network interface info.

* **Socket/BPF programs:** get `struct __sk_buff *skb`

  * Gives access to packet headers and payload.

* **Tracepoints/kprobes:** get `struct pt_regs *regs`

  * Registers and function arguments at the probe point.

* **LSM hooks:** get a ptr to the obj being checked, ex: `struct inode *inode` or `struct task_struct *task`.

**Mechanism:**

* The context pointer is passed in **register R1** (first argument register in eBPF calling convention).
* eBPF uses **helper functions** to safely read or modify data in the context.

---

### **3. eBPF Register Convention**

When the kernel invokes the program:

* R0 → used for **return value**
* R1 → **context pointer** (main input)
* R2–R5 → additional arguments for **helper calls**
* R6–R9 → callee-saved
* R10 → **frame pointer** (stack base)

So the kernel essentially sets up a **register frame**, like this:

```
R0: return value
R1: pointer to context struct
R2-R5: unused (unless calling helpers)
R6-R9: saved registers
R10: stack pointer
```

Then execution jumps to the first eBPF instruction.

---

### **4. Accessing Data in the eBPF Program**

eBPF cannot access kernel memory directly. Instead:

1. **Use the context pointer**:

   * Example: in XDP, to get packet start/end:

     ```c
     void *data = (void *)(long)ctx->data;
     void *data_end = (void *)(long)ctx->data_end;
     ```
   * The kernel ensures these pointers are **bounded to valid memory**, verified at load time.

2. **Use helper functions**:

   * Predefined kernel helpers provide safe access.
   * Examples:

     * `bpf_map_lookup_elem()`: get value from a map
     * `bpf_tail_call()`: jump to another eBPF program
     * `bpf_redirect()`: send packet to a different interface

---

### **5. Return Values**

When the eBPF program finishes:

* It places the **return value in R0**.
* The kernel interprets this return value according to the hook:

  * **XDP:** `XDP_DROP`, `XDP_PASS`, etc.
  * **tc:** return action for traffic control
  * **Tracepoint:** usually ignored
  * **LSM hooks:** enforce security decision

The kernel then continues execution, using the return value to determine the next action.

---

### **6. Summary of Mechanism**

1. Kernel reaches a hook → checks for attached eBPF program.
2. Kernel sets up the eBPF **register frame**.
3. Kernel passes **context pointer** in R1 (hook-specific struct).
4. eBPF program executes, accessing data via context or helper functions.
5. Program finishes → return value in R0.
6. Kernel continues based on return value.

---

💡 **Analogy:**
Think of the kernel as a teacher passing a **worksheet (context pointer)** to a student (eBPF program). 
The student can read/write safely using **allowed tools (helpers)**, and submits their answer in the 
**first box (R0)**. The teacher then acts based on the answer.

---



## eBPF Context Structures:

Understanding of  context struct:
1.  Context structs are defined in kernel. 
    These structures are related to network (xdp), probe ( for functions kprobe) , tracepoints  
    And the contents of the struct's depends on the related program type.

2. When the kernel executes eBPF programs it has to pass arguments to it as input which can be
   network packets, or CPU register values or system call arguments. 

3. By design  eBPF VM mandates that the first arg passed to eBPF prog must be a pointer to this
context structure ( r1 register contins the address of the context) 


### **Step-by-Step Understanding of eBPF Context Structs**

#### **1. Context structs are kernel-defined**

* Correct: These structs are defined in the kernel.
* Their **contents depend on the type of eBPF program**:

  * **XDP:** `struct xdp_md` → points to packet data, metadata (interface index, packet start/end)
  * **Socket programs / tc:** `struct __sk_buff` → gives access to packet headers and payload
  * **Tracepoints:** kernel-defined `struct` with event data (can be autogenerated by tracepoint headers)
  * **kprobes/uprobes:** `struct pt_regs` → CPU register state at function entry/exit
  * **LSM hooks:** pointers to kernel objects like `struct inode`, `struct task_struct`

This allows the eBPF program to **access relevant runtime information** safely.

---

#### **2. Kernel passes program arguments via context**

* Correct: The kernel passes **data as input**, depending on program type:

  * **Packets** (network programs)
  * **CPU registers / function args** (kprobes/uprobes)
  * **System call args or kernel objects** (LSM hooks)
* The context struct **wraps all this data into a single pointer**, so the eBPF VM can operate consistently across different hooks.

---

#### **3. eBPF calling convention**

* Correct: The **first argument to any eBPF program is a pointer to the context struct**.

  * This pointer is placed in **register R1** when the kernel invokes the program.

* The eBPF program **cannot access kernel memory directly**—it uses the context pointer with **helpers** 
  or safe memory operations.
* Other registers (R2–R5) are only used when calling **helper functions**, and R0 is reserved for **return value**.

---

**In short:**

1. Context structs encapsulate all relevant data for a specific eBPF program type.
2. Kernel passes this data via a pointer (context) when invoking the program.
3. eBPF programs follow the **R1 convention** for context input, ensuring uniform access across different hooks.

---

## - eBPF Invokation and Context flow:


```
    [Kernel Hook Triggered] 
            |
            v
    [Check for attached eBPF program]
            |
            v
    [Set up eBPF registers]
      R1 = pointer to context struct
      R0 = return value (uninitialized)
      R2-R5 = helper args (unused initially)
            |
            v
    [Invoke eBPF Program]
            |
            |-- Access context via R1 pointer:
            |       - XDP: struct xdp_md (packet data, interface info)
            |       - Socket: struct __sk_buff (headers, payload)
            |       - Tracepoint: event-specific struct
            |       - kprobe/uprobes: struct pt_regs (CPU registers)
            |       - LSM hooks: kernel objects (inode, task_struct)
            |
            |-- Call helper functions as needed
            |
            v
    [Program finishes execution]
            |
            v
    [Return value in R0 read by kernel]
            |
            v
    [Kernel takes action based on hook type]
```

---

**Explanation of flow:**

1. Kernel triggers a hook (e.g., network packet arrives, function called).
2. Kernel sets up the eBPF VM registers, passing the **context pointer in R1**.
3. eBPF program executes, safely accessing data via the context and helpers.
4. Kernel reads **R0** to decide what to do next (e.g., drop packet, pass, trace event).


##  Kernel Hook Trigger and mechanism of context data :


### **1. Kernel Hook Triggered**

When the kernel encounters an event where an eBPF program is attached (ex: a packet arrives, a function is 
called, a system call executes), it pauses its normal flow and executes the associated eBPF program.

The data passed to the eBPF program is precisely the event data: network packets, CPU register values, 
or system call arguments.

This input data is what the eBPF program uses to make its decisions (e.g., Should I drop this packet? 
What was the second argument to this function?).

i.e: When a kernel event occurs (ex: network packet arrival, function call, system call, file operation):

* The kernel reaches the **hook point** corresponding to the eBPF program type.
* Example hooks:

  * **XDP / tc:** network stack
  * **kprobe / tracepoint:** function entry/exit
  * **LSM:** security-related object access

---

### **2. Check for Attached eBPF Program**

* Kernel maintains a **list or pointer of programs attached** to each hook.
* It first checks:

  1. Is there **any eBPF program attached** to this hook?
  2. If yes, it selects the **correct program** (or the chain of programs, in some hooks like cgroup/LSM).
* If **no program is attached**, kernel just continues normal execution.

---

### **3. Prepare eBPF Execution Context**

Once the kernel finds an attached program:

1. **Context struct creation or reference**

   * The kernel prepares a **context struct** containing relevant runtime information:

     * Packet metadata (`struct xdp_md` / `struct __sk_buff`)
     * CPU registers (`struct pt_regs`)
     * Kernel objects (`struct inode`, `struct task_struct`)
   * This struct is **specific to the hook type**.

2. **Set up eBPF registers**

   * According to eBPF calling convention:

     * **R1** = pointer to the context struct
     * **R0** = return value placeholder (initially 0)
     * **R2-R5** = reserved for helper arguments
     * **R6-R9** = callee-saved
     * **R10** = stack/frame pointer

---
This is the most crucial low-level detail: 

The eBPF Virtual Machine, has register-based machine with ten 64-bit registers (r0 to r9) available for 
program use.
The calling convention for an eBPF program is simple and strict: 
    - Input: Kernel guarantees that the addr (ptr) of the context structure will be loaded into register `r1`
    - Output: The program's return value (ex: XDP_PASS or 0 for success) must be placed in register `r0`.

This standardization ensures that every eBPF program, regardless of its type, has a fixed, reliable way to 
access its input data from the kernel.


### **4. Invoke the eBPF Program**

* The kernel **jumps into the eBPF VM** at the first instruction.
* The program executes in **kernel context**, accessing data **only via the context pointer and helper functions**.
* The verifier ensures the program **cannot crash the kernel**.

---

### **5. eBPF Program Returns**

* When execution finishes:

  * The return value is stored in **R0**.
  * The kernel interprets this return value according to the hook:

    * **XDP:** `XDP_PASS`, `XDP_DROP`, etc.
    * **tc:** traffic control action
    * **LSM:** allow/deny
* The kernel continues execution based on that result.

---

### **6. Summary of Kernel Actions**

At the hook:

1. Kernel checks for an attached eBPF program.
2. Kernel selects the correct program(s) for this hook.
3. Kernel sets up **context struct** and **registers** (R1–R10) for the eBPF VM.
4. Kernel invokes the eBPF program.
5. Kernel reads **R0** and continues execution.

---

**Analogy:**
Think of the kernel as a teacher: when a “trigger event” happens, the teacher checks if a 
“student (eBPF program)” is present. If yes, the teacher hands the student a **worksheet (context struct)** 
and a **pencil (R1 register)**. 
The student completes the task and submits the answer in **R0**, and the teacher acts based on that answer.

