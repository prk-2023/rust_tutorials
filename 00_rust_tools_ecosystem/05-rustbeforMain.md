# Rust before main:


- ref: https://www.youtube.com/watch?v=q8irLfXwaFM
- ref: https://github.com/johnthagen/min-sized-rust/tree/main

## What happens to a rust program before it reaches main():

- Steps and Questions: ( Scope is specific to Linux )

    -> Steps: When we run a rust program till it reaches main(). And what happens at the OS level
    -> How do we get to the main function?
    -> What is a Rust executable?
    -> Tools to inspect the executable.

Recap before we start to look into what things that happen to a program from start to till it reach main().

### Mental Model:

A simple mental model of a binary file :

An executable is thought of a set of instructions packed into a binary file:

    00100010 | 01001000 | 01000100 | 00110010
    0010001001001000010001001001110
    0010001001001000010001001001000 =>  ld a, B
    0010001001001000010001001001011     jmp +50
    0010001001001000010001001001000     add a, 5
    0010001001001000010001001001110
    ..

Most simplified understanding is the file contains instructions and execution that get executed. 
This simplified execution model is not wrong but is very simple and there is a lot under the hood.
A full picture will help to understand the process of execution:

## Rust compilation model:

- source => rustc + llvm  => object files => ld => executable.

General approch to build and run the program is handled by cargo build tool.
cargo presents the code to the "rustc + llvm" for building the project.

rustc: 
    It's the front end that takes the code and performs tasks, presenting an output to llvm back-end.

llvm:
    llvm takes the presented code and convets it into an object file.

object file:
    Object files are group of functions, metadata ... which can all be in one object file or be split into
    multiple objest files.
    obj files are not executable and have to be linked together.

linker:
    'lld' linker takes these objects and generate the executable or generate a library or shared library. 
    When you compile a Rust program using the rustc compiler, it will use the lld linker to link the object.

'lld' : 
    performs the below tasks:
    1. resolves external references: resolves the references the object file has to functions and variables
       defined in other object files or libraries. It does this by searching for definetions in other object
       files and libs.
    2. Links objs: combines objects into a single executable file, resolving any external reference in the
       process.
    3. Assigning memory addresses: Linker assignes addresses to the code and data in the executable file,
       which allows the program to load into memory and execute by the operating system.
    4. Perform re-allocation: When a linker assigns memory address it also perfoms relocation, which
       involves adjusting the code and data to match the assigned addresses.

## Example program:

    ```rust 
        fn main () {
            std::process::exit(code: 0)
        }
    ```

In this sample program with main function: exits with exit code "0".
Since rust optimizes for execution speed, compilation speed and ease of debugging rather then binary size,
but for a vast majority of this additional code is dorment and gets 
Building this code using cargo generates a binary that is large about  3 - 4 MB.

The big size makes it difficult to inspect, it would be nice if we could have a smaller rust program.
Size reduction can be achived via Cargo.toml file:

    [profile.release]
    opt-level = "z"     # Optimize for size.
    lto = true          # Enable Link Time Optimization 
    codegen-units = 1   # Reduce number of codegen units to increase optimizations. 
    panic = "abort"     # Abort on panic 
    strip = true        # Automatically strip symbols from the binary.

    With this we can reduce the size of the binary by ~ 93% 

- https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles
    Cargos default opt-level is set to 3, which optimized the binary for speed.
    For generating small size use "z" optimization level.

- lto = true #link time optimizations 
    (https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles)

    By default cargo instructs compilation unit to be compiled and optimized in isolation.

- Reduce parallel code generation units to increases optimization:

    By default cargo specifies 16 parallel codegen units for release build. this improved compile time but
    it prevents some optimizations. Set `codegen-units = 1` for max size reduction optimization.

- "abort" on panic (https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles)

    By default when Rust code encounters a situation when it must call panic!(), it unwinds the stack and
    produces a helpful backtrace. This unwinding code, however does require extra binary size.

    rustc can be instructed to abort immediately rather then unwind, which removed the need for this extra
    unwinding code.

    [profile.release]
    panic = "abort"

NOTE: Comparing this with a most striped down binary size of same C code would generate a binary ~ 16 kb.

## More on size control:

Further reduction of the size can be achived by removing the preludes  and other programs that gets packed 
with the binary on compilation:

    ```rust 
    #![no_std]
    #![no_main]

    extern crate libc;  // If this code cant be compiled include the below lines to Cargo.toml
    // [dependencies]
    // core = { version = "0.0.0", features = ["compiler-builtins"] }

    #[no_mangle]
    pub extern "C" fn main() -> isize {
        0
    }

    #[panic_handler]
    fn my_custom_panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }
    ```
- #![no_std] :
    This attribute tells the Rust compiler not to link to the Rust standard library (std). 
    Used when writing code that needs to be highly optimized or when working with embedded systems.

- #![no_main] :
    This attribute tells the Rust compiler not to generate a main function. 
    Instead, the program will use a custom entry point.

- extern crate libc;
    The extern crate libc; tells the Rust compiler to link to the libc crate, which provides a Rust 
    interface to the C standard library.

- #[no_mangle] :
    The attribute tells the Rust compiler not to mangle the name of the main function. 
    This is necessary because the main function is being used as the entry point of the program, and its 
    name needs to be recognizable by the linker.

- pub extern "C" fn main() -\> isize { ... }

    The main function is defined as a public, external function that takes no arguments and returns an 
    isize value. 

    The extern "C" keyword indicates that the function should be compiled using the C calling convention.

- #[panic_handler] :
    This attribute tells the Rust compiler that the "my_custom_panic" function should be used as the panic 
    handler for the program.

    ` fn my_custom_panic(_info: &core::panic::PanicInfo) -\> ! { ... } `

    function that takes a " &core::panic::PanicInfo " argument and returns a "!" value 
    (which indicates that the function will never return). 
    The function is used to handle panics in the program.


## What is Binary executable:

- hexbomb target/release/hello | head -n 3
 ·······0 │ 7F 45 4C 46 02 01 01 00 ┆ 00 00 00 00 00 00 00 00 │ ·ELF····┆········ |

or 
- hex_dump target/debug/small  |head -n 3
 Adress   0   1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
00000000  7F 45 4C 46 02 01 01 00 00 00 00 00 00 00 00 00  |.ELF............|
00000010  03 00 3E 00 01 00 00 00 50 67 00 00 00 00 00 00  |..>.....Pg......

or use the file command as:

$file  target/release/hello
target/release/hello: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, 
interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=6cce3cf180f969a02abad9f428803e616d3985e6, for 
GNU/Linux 3.2.0, not stripped

The ELF: Executable and linkable format: 

    +-----------------------+
    |  ELF Header           |
    +-----------------------+
    | Prog header table     |
    +-----------------------+
    |      .text            |
    +-----------------------+
    |      .rodata          |
    +-----------------------+
    |       ....            |
    +-----------------------+
    |       .data           |
    +-----------------------+
    |section header table   |
    +-----------------------+


- ELF: the first line of a executable binary has the indicator "ELF" as seen using the hex Tools

- ELF has information of both for static linking and runtime execution.

- the object file which are formed during compilation are also ELF format and they have runtime information
  the executable has. 

- ELF is a data base that allows us to lookup the info we need to run our program.

- The .text section contains our code.

NOTE: the common debug data format in Linux is called DWARF format and ELF and DWARF are related to each
other.

###  Where is the code.

- llvm-objdump -d target/debug/small | grep main
    6764: 48 8d 3d 85 01 00 00          leaq    389(%rip), %rdi         # 0x68f0 <main> 
    00000000000068e0 <_ZN5small4main17h6de225dbb54097ceE>:
    00000000000068f0 <main>:

  -d : disassembel the code inside the binary.

- When we look into the output of llvm we can see there is a lot more code then just the main function we
  have written.

- the other part of the code shows that main is not the entry point of the code. 

- The <_start> symbol is the real entry point of the program:

   1724 0000000000006750 <_start>:
   1725     6750:»  31 ed                »  xor    %ebp,%ebp
   1726     6752:»  49 89 d1             »  mov    %rdx,%r9
   1727     6755:»  5e                   »  pop    %rsi
   1728     6756:»  48 89 e2             »  mov    %rsp,%rdx
   1729     6759:»  48 83 e4 f0          »  and    $0xfffffffffffffff0,%rsp
   1730     675d:»  50                   »  push   %rax
   1731     675e:»  54                   »  push   %rsp
   1732     675f:»  45 31 c0             »  xor    %r8d,%r8d
   1733     6762:»  31 c9                »  xor    %ecx,%ecx
   1734     6764:»  48 8d 3d 85 01 00 00 »  lea    0x185(%rip),%rdi        # 68f0 <main>
   1735     676b:»  ff 15 97 f2 04 00    »  call   *0x4f297(%rip)        # 55a08 <__libc_start_main@GLIBC_2.34>
   1736     6771:»  f4                   »  hlt
   1737     6772:»  66 2e 0f 1f 84 00 00 »  cs nopw 0x0(%rax,%rax,1)
   1738     6779:»  00 00 00·
   ...

- <_start> :
    - setting up the stack 
    - move argc and argv to the right place 
    -  prepares the call to : __libc_start_main:
        1735     676b:»  ff 15 97 f2 04 00   call  *0x4f297(%rip)   # 55a08 <__libc_start_main@GLIBC_2.34>
    -  we can see the assembly "call" which calls the "__libc_start_main"

- __libc_start_main: function:
    - performs initalization of the execution environment
    - calls the main funtion with appropriate arguments 
    - handles the return from main()
    And this function also does some other things.

    - calls various hooks that allow the user to change startup and exit:
        - __libc_csu_init 
        - __libc_start_fini
    This is where we can define things that we wish to run before or after the program exits.

    -  As of this writing there is not a stable way to do this in rust.

- dependencies:

    ldd  target/debug/small
        linux-vdso.so.1 (0x00007ffcf95e2000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f27646ad000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f27644cc000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f276473f000)

    linux-vdso.so.1 : provides to all programs for faster systemcalls. 

    Note: these libraries are not part of the excutables but are located in other libraries and our program
    is dynamically referencing them ( i.e dynamically linked shared libs )

    Most cargo built programs have dependencies that are statically linked then shared.

- recap of things that are before main()

    <_start> --> __libc_start_main --> main()

- so we can replace the code for the small program as below:

    ```rust 
    #![no_std]
    #![no_main]


    #![no_mangle]
    pub extern "C" fn _start() -> ! {
        loop{}
    }

    #[panic_handler]
    fn my_panic (_info: &core::panic::PanicInfo) -> {
        loop {}
    }
    ```
    This program tells that we take care of how to start the program and requires special compilation
    options to cargo (to pass the error by the linker that we have two _starts, this is because the linker
    by default put a _start for us and we need to disable that):

    "cargo rustc --release -- -C link-arg=-nostartfiles" 

    this will now generate a binary that same in size with C.

    This compilation options tell we will take care of the starting and not to attach a _start by the
    linker.

### _start

- why do we start at _start?

    When we look at the header of the executable:

    ( for the relase version)
    $readelf -h  target/release/small
    ELF Header:
      Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
      Class:                             ELF64
      Data:                              2's complement, little endian
      Version:                           1 (current)
      OS/ABI:                            UNIX - System V
      ABI Version:                       0
      Type:                              DYN (Position-Independent Executable file)
      Machine:                           Advanced Micro Devices X86-64
      Version:                           0x1
      Entry point address:               0x1000      <======= This is the entry point
      Start of program headers:          64 (bytes into file)
      Start of section headers:          12432 (bytes into file)
      Flags:                             0x0
      Size of this header:               64 (bytes)
      Size of program headers:           56 (bytes)
      Number of program headers:         10
      Size of section headers:           64 (bytes)
      Number of section headers:         11
      Section header string table index: 10

    for debug version build:
    readelf -h target/debug/small
    ELF Header:
      Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
      Class:                             ELF64
      Data:                              2's complement, little endian
      Version:                           1 (current)
      OS/ABI:                            UNIX - System V
      ABI Version:                       0
      Type:                              DYN (Position-Independent Executable file)
      Machine:                           Advanced Micro Devices X86-64
      Version:                           0x1
      Entry point address:               0x6750
      Start of program headers:          64 (bytes into file)
      Start of section headers:          3830496 (bytes into file)
      Flags:                             0x0
      Size of this header:               64 (bytes)
      Size of program headers:           56 (bytes)
      Number of program headers:         14
      Size of section headers:           64 (bytes)
      Number of section headers:         41
      Section header string table index: 40

    The ELF entry point address is where we start the program execution. i.e where in the binary the kernel
    should start executing this program.

    So if we look in the elf file at address 0x6750 we will see that is where the programs _start would be
    located.

    ...
    0000000000006750 <_start>:
      6750:»  31 ed                »  xor    %ebp,%ebp 
      6752:»  49 89 d1             »  mov    %rdx,%r9
      6755:»  5e                   »  pop    %rsi
      6756:»  48 89 e2             »  mov    %rsp,%rdx
      6759:»  48 83 e4 f0          »  and    $0xfffffffffffffff0,%rsp
      675d:»  50                   »  push   %rax
      675e:»  54                   »  push   %rsp
      675f:»  45 31 c0             »  xor    %r8d,%r8d
      6762:»  31 c9                »  xor    %ecx,%ecx
    ...

- _start is name is the convention that the linker takes, but from operating system point its not aware that
  the _start exists or cares for it and considers as just a label. 

### who calls _start?

- some times its the kernel that calls _start, but we need additional info before getting started. The
  additional things that are required before hitting _start are some services that parse the ELF binary and
  map its parts into memory for us so the code from the disk gets read and is loaded to somewhere in the
  RAM. Which is why we make an executable as memory regoins are not treated the same  some memroy is
  readable and some is writable and some is executable and this requires us to make every thing of the
  program lives in the righr place.

  -> parse the elf binary and map into memory 
  -> loads dynamic dependencies & ensure the binary can call them
  -> how to make sure the libraries arr in our address space.
  -> We need a way to load these dependencies
  -> also need to ensure that the binary can call them

  this loading is done by a "dynamic interpreter" or "dynamic loader", or "static linker" whos jobs is :
  - take the binar and map into memory
  - check the dynamic dependencies and load them and ensures the binary can call them
  The kernel can perform some of these tasks, where the kernel starts and where the dynamic interpreter
  starts .. ( refer more on this Out of scope for this article.) 

### The dynamic linker:

    readelf -x .interp  target/debug/small

    Hex dump of section '.interp':
      0x00000350 2f6c6962 36342f6c 642d6c69 6e75782d /lib64/ld-linux-
      0x00000360 7838362d 36342e73 6f2e3200          x86-64.so.2.

    this shows that the binary needs /lib64/ld-linux-x86-64.so.2

    This is what the ldd thrid dependencie we required by the binary. And its the kernel that puts some of
    the executable into memory and looks what is required and informs the the dynamic loader to do the
    copying. 

    dynamic Lodare ==> map memory load deps relocations ==> jump to _start 

    ( there is a lot going here by the dynamic loader in the fist step and can be probed for more 
    info on how its loaded and how is mmaped and ... more )


### How do we invoke the dynamic loader?

- The execve sys call

    - execve() executes the program referred to by PATH
    the kernel looks at the executable and finds the interpreter named and loades that loader and hands off
    the task to the loader from there on.
    - PATH must either be a binary executable or a script starting with #!.
    - if the executable is a dynamically linked ELF executable, the interpreter named in the PT_INTERP
      segment is used

    execve ==>
        kernel loaded the dynamic loader ==> 
            which then maps and loads and calls ==> 
                _start ==>
                    __libc_start_main ==>
                        main().

