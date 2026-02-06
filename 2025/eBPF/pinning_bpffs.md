# BPF Virtual FileSystem (bpffs) 


Its a pseudo file-system,  used to make the BPF objects persistent. 

Normally BPF programs and maps are tied to a file-descriptor (FD) of the process that created them. If that
process terminates or closed the FD, the BPF object will be deleted by the kernel. 

`bpffs` solves this by allowing you to "pin" these objects to a file path, keeping them alive even if the
original process exits. 

BPF objects (maps, programs) are governed by a reference counting system.

- When a process creates a BPF object via the bpf() syscall, the kernel initializes its reference count to 1
  and returns a File Descriptor (FD).
  - As long as the FD is open the count stays at least at 1.
  - If the process crashes or closes the FD, the count drops to 0 and kernel immediately deallocates the
    object. 

- Pinning Action: 

    When you "pin" and object to the BPF virtual fs, you are telling the kernel "create a new persistent
    reference to this object."
    - The kernel increments the reference count + 1 , by creating an entry in `/sys/fs/bpf/`


- mounting FS:

    `mount -t bpf bpffs /sys/fs/bpf`


- When a BPF program creates a map (for storing stats, configuration, etc.), it can "pin" that map to a
  location like `/sys/fs/bpf/my_map`.
  - The kernel increments a reference counter for that object.
  - Other independent processes can then "peek" into that map or attach to that program by simply opening
    that file path.

- Multiple tools can read from the same BPF map. 
  Example: one program might collect packet counts, while another might be monitoring tool it reads those 
  counts from the pinned file in `/sys/fs/bpf`.

- `bpftool` can interact with loaded program via the filesystem.

- Create a pin a Map:

  Ex: Create my_test_map hash map with key:value and max of 10 entries.

  ```bash 
  # bpftool map create /sys/fs/bpf/my_test_map type hash key 4 value 4 entries 10 name my_test_map
  # ls -l /sys/fs/bpf/my_test_map
  -rw------- 1 root root 0 Oct 24 10:00 /sys/fs/bpf/my_test_map
  ```

- Check map and reference count:
    `sudo bpftool map show name my_test_map`

   `ID`: unique identifier the kernel assigned.
   `frozen 0`: indicates the map is still modifiable.
   `pinnable`: confirms the map can be ( and is ) held by the filesystem. 


- Delete the map,program: 
    `sudo rm /sys/fs/bpf/my_test_map`
    `sudo bpftool map show id 42` <= if no other process opens that map, bpftool returns nothing and
    reference count drops to 0, and kernel automatically reclaims the memory.

- Pinning is done by bpftool or the loading program using `bpf()` call.
    - User-space calls 
        bpf(BPF_MAP_CREATE, ...) or bpf(BPF_PROG_LOAD, ...). 
      The kernel returns a File Descriptor (FD).

    - User-space calls bpf(BPF_OBJ_PIN, ...) and provides the FD and the desired path in `/sys/fs/bpf`.
      This is when the reference count increments.

- using `C` (libbpf) 
    ```C 
    // Pinning a map
    bpf_map__pin(map, "/sys/fs/bpf/my_persistent_map");
    
    // Pinning a program
    bpf_program__pin(prog, "/sys/fs/bpf/my_persistent_prog");
    ```
- using `bpftool`:

```bash
    # Loading and pinning a program in one go
    sudo bpftool prog load my_prog.o /sys/fs/bpf/my_prog
```

- Pinning using `aya`

```rust 
    use aya::maps::HashMap;
    use std::convert::TryFrom;

    // 1. Load the BPF object
    let mut bpf = Bpf::load(include_bytes_aligned!("../../target/bpfel-unknown-none/debug/myapp"))?;

    // 2. Get a reference to the map
    let mut my_map = HashMap::try_from(bpf.map_mut("MY_MAP").unwrap())?;
    
    // 3. Pin it to the BPF filesystem
    // This triggers the BPF_OBJ_PIN syscall and increments the refcount
    my_map.pin("/sys/fs/bpf/my_shared_map")?;
```

- Pinning a program `aya`: Pinning program is similar, this is useful if you want the program to stay
  attached to a hook even if the demon/User-space restarts:

```rust 
use aya::programs::Xdp;

let program: &mut Xdp = bpf.program_mut("my_xdp_prog").unwrap().try_into()?;
program.load()?;
// Pinning the program keeps the executable code alive in the kernel
program.pin("/sys/fs/bpf/my_xdp_prog")?;
```

NOTE: In Rust if we run the program twice, the file `/sys/fs/bpf/my_test_map` already exists and `pin()` can
return `Error (EEXIST)`. And its important to handle this properly, we check if the pin exists and reuse it
instead of creating a new one.

Aya follows *RAII* ( Resource acquisition is initialization) pattern.
Normally, when your `Bpf struct` goes out of scope in Rust, Aya automatically closes the file descriptors,
which would drop the refcounts to 0. 

By calling `.pin()`, you are explicitly opting out of that automatic cleanup for that specific object.



