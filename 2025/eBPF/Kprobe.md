# kprobe:

## 1. What Kprobes IS:

- **Dynamic kernel instrumentation** - Insert debug points at runtime.
- **Breakpoint-based** - Uses CPU hardware breakpoints (int3 on x86).
- **Zero-overhead when idle** - No performance cost until triggered.
- **Exception-driven** - Triggers via CPU traps, not software checks.

## 2. What Kprobes is NOT
- NOT a "check before each instruction" system
- NOT a polling mechanism
- NOT a traditional hook/callback system
- NOT a software-only solution

---

## 3. Core Mechanism - The 4-Step Process

### Step 1: Instruction Hijacking
```
Original: [mov %rax, %rbx]   ← Normal instruction
Probed:   [int3]             ← Breakpoint replaces it
```

### Step 2: Trap Generation
- CPU executes `int3` → Hardware exception (trap #3)
- Automatic switch to kernel mode
- Control goes to debug exception handler

### Step 3: Handler Execution
```c
debug_exception_handler() {
    if (address_has_kprobe())
        kprobe_handler();    ← YOUR callback runs here
}
```

### Step 4: Resume
- Execute original instruction (single-step or emulate)
- Continue normal execution

---

## 4. Key Components Explained

### A. Probes (The "What")
```c
struct kprobe {
    kprobe_opcode_t *addr;      // WHERE to probe
    kprobe_pre_handler_t pre_handler;  // WHAT to run before
    kprobe_post_handler_t post_handler; // WHAT to run after
    // ... other fields
};
```

**Types:**
1. **Kprobe** - Single instruction probe
2. **Jprobe** - Function entry probe (deprecated)
3. **Kretprobe** - Function return probe

### **B. Handlers (The "Hooks")**
- **`pre_handler`** - Runs BEFORE original instruction
  ```c
  int my_pre_handler(struct kprobe *p, struct pt_regs *regs)
  ```
  - Can inspect/modify registers
  - Can skip original instruction (rare)

- **`post_handler`** - Runs AFTER original instruction
  - For cleanup or logging results

- **`fault_handler`** - Runs if error occurs
  - For error recovery

### **C. Registration (The "How")**
```c
// 1. Define probe
struct kprobe kp = {
    .symbol_name = "do_fork",  // Function name
    .pre_handler = my_handler,
};

// 2. Register it
int ret = register_kprobe(&kp);  // ← Insertion happens HERE

// 3. Use it (automatically triggers)

// 4. Clean up
unregister_kprobe(&kp);  // ← Removal
```

**Registration actions:**
- Validates address
- Saves original bytes
- Writes breakpoint
- Adds to probe table

---

## **5. The "Hook" Misconception Clarified**

### **Traditional Hook:**
```c
// Code is HOOK-AWARE
void kernel_function() {
    do_work();
    if (registered_hook)     ← EXPLICIT CHECK
        registered_hook();   ← EXPLICIT CALL
}
```

### **Kprobe "Hook":**
```c
// Code is UNAWARE
void kernel_function() {
    do_work();  // No checks, no calls
    // ↑ If breakpoint here, magic happens
}
```

**Key difference:**
- **Hook** = Code intentionally calls you
- **Kprobe** = You intercept code execution

---

## **6. Real Registration Flow**

```
User: register_kprobe(&kp)
     ↓
Kernel: save original instruction at address
     ↓
Kernel: replace with breakpoint (int3)
     ↓
Kernel: add to internal hash table
     ↓
DONE - Probe is active!
```

**When triggered:**
```
CPU: Executes int3 → TRAP!
     ↓
Kernel: Exception handler runs
     ↓
Kernel: Looks up address in hash table
     ↓
Kernel: Finds kprobe → calls pre_handler
     ↓
User: Your code runs!
     ↓
Kernel: Single-step original instruction
     ↓
Kernel: Resume normal execution
```

---

## **7. Memory View**

**Before registration:**
```
Address 0xffff1234: [48 89 d8]      mov %rbx, %rax
```

**After registration:**
```
Address 0xffff1234: [cc]            int3
Saved original:    [48 89 d8 90 90] mov %rbx, %rax + nops
```

**When hit:**
1. CPU reads `cc` → trap
2. Handler knows 0xffff1234 has kprobe
3. Your handler runs
4. CPU executes saved `[48 89 d8]` (single-step)
5. Continue from 0xffff1237

---

## **8. Important Features**

### **A. Transparency**
- Kernel code unaware of probes
- Can probe ANY instruction (almost)
- Removal restores original code

### **B. Safety Features**
- Prevents recursive probes
- Validates probe points
- Handles SMP concurrency
- Protection against probing critical code

### **C. Data Access**
```c
// In handler:
unsigned long value = regs->ax;  // Read register
printk("RAX = %lx\n", regs->ax); // Log it
regs->ax = new_value;            // Modify it (careful!)
```

### **D. Performance**
- **No probe hit**: 0% overhead (just 1 extra byte fetch)
- **Probe hit**: High overhead (trap + single-step ≈ 1000+ cycles)
- **Optimized probes**: Use jumps instead of traps for hot paths

---

## **9. Common Use Patterns**

### **Pattern 1: Function Tracing**
```c
probe.symbol_name = "sys_open";
probe.pre_handler = log_call;
// Logs every sys_open call
```

### **Pattern 2: Argument Inspection**
```c
// In handler:
char *filename = (char *)regs->di;  // First arg on x86
printk("Opening: %s\n", filename);
```

### **Pattern 3: Return Value Capture**
```c
// Using kretprobe:
struct kretprobe rp = {
    .kp.symbol_name = "kmalloc",
    .handler = kmalloc_return,
};

int kmalloc_return(...) {
    long size = regs_return_value(regs);
    printk("kmalloc returned %ld bytes\n", size);
}
```

### **Pattern 4: Conditional Breakpoint**
```c
int handler(...) {
    if (regs->di == 0xdeadbeef)  // Condition
        printk("Special case!\n");
    return 0;
}
```

---

## **10. Summary Cheat Sheet**

| Concept | In Kprobes | Analogy |
|---------|------------|---------|
| **Hook** | Your handler function | Burglar alarm callback |
| **Registration** | `register_kprobe()` | Installing the alarm |
| **Trigger** | Breakpoint trap | Alarm sensor tripping |
| **Probe Point** | Instruction address | Where to place sensor |
| **Handler** | Your callback code | What to do when alarm goes off |
| **Single-step** | Executing original instruction | Letting the normal activity continue after checking |

**Kprobes in one sentence:**
> *"A system that replaces kernel instructions with breakpoints to trigger your code via CPU exceptions,
> then carefully executes the original instruction and resumes."*

**Key takeaway:**
Kprobes don't ADD checks to the kernel—they REPLACE instructions temporarily, letting the HARDWARE do the
checking for free!

------------------------------------------------------------------------------------------------------------
# Compair Kprobe with XDP:


Let's break it down and visualize the differences between **Kprobe** and **XDP** using kernel code examples 
and text-based flow diagrams.

### 1. **Kprobe Example (Kernel Probe)**

**Purpose**: 
    Insert a probe at a specific kernel func to capture events, inspect or modify kernel behavior during runtime.

**Kernel Code Example**:

```c
#include <linux/kprobe.h>
#include <linux/module.h>
#include <linux/init.h>

static struct kprobe kp = {
    .symbol_name = "sys_clone",  // Function to probe (sys_clone in this case)
    .pre_handler = kprobe_handler,  // Pre-handler function when the probe is hit
};

// Pre-handler function to be invoked on each probe hit
static int kprobe_handler(struct kprobe *p, struct pt_regs *regs)
{
    printk(KERN_INFO "Kprobe triggered! Process ID: %d\n", current->pid);
    return 0;  // Continue execution of the probed function
}

static int __init kprobe_init(void)
{
    int ret;
    
    // Register the probe
    ret = register_kprobe(&kp);
    if (ret < 0) {
        printk(KERN_ALERT "Registering kprobe failed\n");
        return ret;
    }

    printk(KERN_INFO "Kprobe registered successfully\n");
    return 0;
}

static void __exit kprobe_exit(void)
{
    unregister_kprobe(&kp);
    printk(KERN_INFO "Kprobe unregistered\n");
}

module_init(kprobe_init);
module_exit(kprobe_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Your Name");
MODULE_DESCRIPTION("Kprobe Example");
```

### Explanation:

* **Kprobe** is registered on the `sys_clone` function (which is responsible for creating processes).
* When the function `sys_clone` is called, the `kprobe_handler` is invoked, logging the Process ID (`PID`).
* The `pre_handler` is used here, meaning the probe is triggered before the actual kernel function is executed.

### Flow Diagram for Kprobe:

```
            +----------------------+
            |  sys_clone function  |  <-- Function to probe
            +----------------------+
                        |
                (Execution of probe)
                        |
            +----------------------+
            |    kprobe_handler    |  <-- Custom handler invoked
            +----------------------+
                        |
           (Inspect/modifies kernel state)
                        |
          +------------------------+
          |    Continue execution  |  <-- Proceed with sys_clone function
          +------------------------+
```

### 2. **XDP Example (eXpress Data Path)**

**Purpose**:

Process network packets directly at the network driver level, before they enter the kernel’s network stack.


**Kernel Code Example**:

```c
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/udp.h>
#include <linux/if_link.h>
#include <linux/netdevice.h>
#include <linux/module.h>

static int xdp_prog_fd = -1;

// BPF program attached to the network interface
SEC("xdp")
int xdp_prog(struct __sk_buff *skb)
{
    // Just an example: Drop the packet (no forwarding)
    return XDP_DROP;
}

char _license[] SEC("license") = "GPL";

static int __init xdp_init(void)
{
    struct bpf_program *prog;
    struct bpf_object *obj;

    // Load the BPF program
    obj = bpf_object__open_file("xdp_prog.o", NULL);
    if (IS_ERR(obj)) {
        pr_err("BPF object load failed\n");
        return PTR_ERR(obj);
    }

    prog = bpf_program__next(NULL, obj);
    bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);

    // Attach the XDP program to a network interface (eth0 here)
    bpf_set_link_xdp_fd(0, bpf_program__fd(prog), XDP_FLAGS_UPDATE_IF_NOEXIST);
    
    pr_info("XDP program loaded successfully\n");
    return 0;
}

static void __exit xdp_exit(void)
{
    // Remove the XDP program from the interface
    bpf_set_link_xdp_fd(0, -1, 0);
    pr_info("XDP program removed\n");
}

module_init(xdp_init);
module_exit(xdp_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Your Name");
MODULE_DESCRIPTION("XDP Example");
```

### Explanation:

* **XDP** uses an eBPF program (`xdp_prog`) attached to a network interface.
* The program drops all incoming packets (by returning `XDP_DROP`), providing high-performance filtering 
  before the packet enters the kernel's networking stack.
* The XDP program is loaded into the kernel and attached to a network device (e.g., `eth0`) at boot or
  module load time.

### Flow Diagram for XDP:

```
+------------------------+
| Network Interface      |  <-- Device that receives packets (e.g., eth0)
+------------------------+
            |
      (Packet arrives)
            |
+------------------------+
| XDP Program (eBPF)     |  <-- XDP program intercepts packet
| (e.g., XDP_DROP)       |  <-- Drops or processes the packet
+------------------------+
            |
    (Decision: Drop/Forward)
            |
+------------------------+
| Network Stack          |  <-- Regular kernel network stack (skipped if dropped)
+------------------------+
```

### Key Differences in Flow:

1. **Kprobe**:

   * Hooks into specific kernel functions (like `sys_clone`).
   * Executes a custom handler when the function is called.
   * It's used for **debugging/monitoring** kernel behavior, **not directly related to networking**.
2. **XDP**:

   * Hooks into the **networking driver layer**, processing packets **before** they hit the kernel's 
     networking stack.
   * Can drop, modify, or redirect packets for performance reasons.
   * It's used for **high-performance packet filtering** and **networking optimizations**.

### Summary:

* **Kprobe** provides **dynamic tracing** for debugging kernel code, giving you the ability to inspect and 
  modify kernel behavior at runtime.
* **XDP** provides **high-speed networking optimizations** by allowing you to process packets at the 
  earliest point in the network stack.

Both technologies serve different purposes but leverage kernel hooks to achieve their goals: **Kprobe** for 
kernel observability and **XDP** for performance-driven network packet processing.

