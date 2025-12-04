1. Prerequisites :
    [O] Linux kernel and Networking Basics 
    [O] Rust Fundamentals 
    [O] What is eBPF?
       [O]  Concepts 
       [O]  Tools ( different tools and methods used for tracing )
       [O]  BTF, CO-RE
2. Build environment:
    [O] Setting up required packages and repos
3.
    [ ] Aya
        [O] Intro
        [O] Aya Modules 
        [ ] Working with BPF Maps
        [ ] Attaching BPF Programs and Hooks
        [ ] Error Handling and Debugging
        [ ] Performance Optimization
        [ ] Async programming  
        [ ] Security Consideration 
    [ ] Building Real-World Projects:
        [ ] Networking and Monitoring 
        [ ] Performance Tracing and Profiling 
        [ ] Security Applications 
        
4. Aya Internals 
    [ ] Aya design and Internals
        [ ] aya release crate
            [ ] lib.rs : A library to work with eBPF programs.
            [ ] bpf.rs : The main entry point into the lib, loading eBPF code, maps and relocations
            [ ] pin.rs : Pinning BPF objects to the BPF filesystem.
            [ ] util.rs: Utility functions
            [ ] sys/{bpf.rs, mod.rs, netlink.rs, perf_event.rs ..} : A collection of system calls for 
                performing eBPF related operations.
            [ ] programs/{cgroup*.rs, kprob.rs, tp.rs, tc.rs, xdp.rs ...} : eBPF program types, that are
                loaded into kernel and attached to one or more hookpoints.
            [ ] maps/{array/, hash_map/, perf/, xdp/, sock/, ring_buf.rs, stack.rs ...} : Data structures 
                used to setup and share data with eBPF programs.
            [ ] 
        [ ] ebpf
            [ ] aya-ebpf: The eBPF Rust runtime ( `no_std`, `no_main` ) (kernel-space counterpart of Aya)     
                [ ] programs/ : eBPF program type modules
                [ ] maps/     : maps related to eBPF program types
                [ ] btf_maps/ : eBPF maps that use BTF metadata to describe their key/value types to the kernel.
            [ ] aya-ebpf-bindings: Bindings for Linux kernel eBPF types and helpers
            [ ] aya-ebpf-macros : proc macros used by aya-ebpf
            [ ] aya-log-ebpf : Logging for eBPF programs.
            [ ] aya-ebpf-cty : Type aliases to C type like c_int for use with bindgen
        [ ] aya-log: Logging framework for eBPF Programs
        [ ] aya-log-parser : Parser for aya log format strings
        [ ] aya-obj: eBPF Object parsing library with BTF and relocation support.
