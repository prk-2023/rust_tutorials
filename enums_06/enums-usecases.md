# Important Enums that are used in programming:

Understanding forms of enums that rust provides helps to effectively use them in programs:

## Forms of Enums

Rust enums represent a sum-type (i.e a type that can hold one of several possible values), allowing you to
define a value that can be one of several possible states.

### Atoms, Sum Types, Product Types, and Generic Types

To understand enums, it's essential to know about the different types in Rust:

*  **Atoms**: 
    basic building blocks of types, such as `i32`, `u64`, `f32`, etc. 
    Atomic types are not composed of any other types and represent single , indivisible valuel

*   **Sum Types**: 
    They allow you to define a value that can be one of several possible states. 
    The number of possible states a variable can have is the sum of all options.
    enums: In rust enums are sum type, which means it can represent a value that can be one of several variants, 
    each potentially hold different data.
    The sum refers to the number of possible values across all variants. ex

    ```rust 
        enum Option<T> {
            Some(T),
            None,
        }
    ```
    So if `T` is `bool` then `Option<T>` has 3 possible states:
    `Some(true)` , `Some(false)` and `None`  ==> 3 possible states.

*   **Product Types**: 
    `Struct` are an example of product types. 
    They define a type that is a combination of multiple values. 
    Number of possible states a variable can have is the product of all combinations of data in the struct.
    
*   **Generic Types**: 
    These are types that are defined in terms of some other type (or types) `T`. 
    They allow for more flexibility and reusability in your code.
    - Rust supports `parametric polymorphism`  via `generics`
    i.e : *`Writing code that works with any any data type.`*
    
    Instead of writing a function or data structure for each specific type (like i32, f64, or String), you 
    write it once and make it generic over types.
 
    In Rust, `generics` let you define functions, structs, enums, or traits that can operate on different 
    types without rewriting the code multiple times.

    These allows us to write types like:
    ```rust 
        struct Wrapper<T> {
            value: T,  // T can be of any type making code reusable and type-safe.
        } 
    ```
 
  This means `Identity` function works doe any type: (i32, u32, &str ).

[ Note: Why is this parametric polymorphism?
  
  Because the behavior of identity is parameterized by the type T, it works uniformly regardless of what 
  type you pass in — the exact type is a parameter of the function.

  Parametric polymorphism = writing functions or data types that work for any type.
  *Generics in Rust are the language feature that implements parametric polymorphism.*
]
This lets you write flexible, reusable code without losing type safety.

 example: 

 ```rust 
    // A generic function that returns whatever it receives
    fn identity<T>(value: T) -> T {
        value  // Just returns the input value of any type T 
    }

    fn main() {
        let int_val = identity(5);        // T inferred as i32
        let str_val = identity("hello");  // T inferred as &str
        println!("int_val = {}, str_val = {}", int_val, str_val);
    }
```
    - `<T>` declares a generic type parameter `T`.
    - identity works for any type T — that’s parametric polymorphism.
    - The function simply returns the input value without caring about its type.
    - Rust infers the specific type when you call it (i32 for 5, &str for "hello").


There are other possible types : traits and functions ...

Rust enums ( which are sum-type ) make most sense when you want to represent a 'choice' as type of something

The most common example in rust is "Option<T>"type, a generic enum defined as:

    pub enum Option<T> {
        Some<T>,
        None,
    }

This enum represents "Some" of some 'T' or "None".

Other enums are a "Choice" in similar way.

Every variable in Enum type can only be one entry in the enum type at a time:

    let x:Option<f32> = Some(3.14);
    let y:Option<f32> = None;

Here x and y have to be something but can not have both values.


So 
```rust 
enum Color {
    Red,
    Green,
    Blue,
}
```
Unlike C, Rust’s type system isn’t entirely based on integers. 
Types have significantly more meaning. 
Instead, Color is a type in your program where `Color::Red` is a distinct value from `Color::Green` is a 
distinct value from `Color::Blue`, in the same way that 0 is distinct from 1 or 2 in C.

[ There are crates that map this enums to integers if we wish to use them as C or C++ ]

### Named Data:

we can give names to the data in emums which can be helpful when enums get passed around:

    enum FrameOfRef {
        x { value: f32 },
        y { value: f32 },
        z { value: f32 },
        t { time: f32 },
    }

    enum PixelFormat {
        Yuv {
            y: u8,
            u: u8,
            v: u8,
        },
        Rgb {
            r: u8,
            g: u8,
            b: u8,
        },
        Greyscale(u16),
        Unknown,
    }
    This is a bit complex way to define, but avoid significant amount of complexity while passing data
    around.

### Option<T>: Rust does not have default or optional args in functions like in C,C++. Which are not required in
Rust, instead we use Option<T> if we require a parameter or not.

```
    fn add_three_numbers(a: i32, b: i32, c: Option<i32>) -> i32 {
        if let Some(value) = c {
            a + b + value 
        } else {
            a + b 
        }
    }
    let x = add_three_numbers(13, 12, None);    // => 25
    let y = add_three_numbers(13, 12, Some(4)); // => 
```


Similarly, if you avoid using unsafe Rust and raw pointers, the default Rust types for ptrs (Box,Arc,etc) 
cannot exist as a “null” pointer. 
Instead, if you want to represent a pointer that can be null, you use Option<Box<T>>, and null pointers are
represented as "None". Unlike in C or C++, this makes the type of pointers very explicit. 

You know that you have to handle the enum differently than you would the pointer itself, and once you get 
a Box<T> you know for certain that it points to something. 
This is one way safe Rust avoids null de-references!

- enum Result<T, E>:
Result is Rust’s standard error type. It is a fairly straightforward enum that roughly takes the form:

    enum Result<T, E> {
       Ok(T),
        Err(E),
    }

Result is usually used as the return value from a function, which tells Ok<T> on success  or it fail 
where you get some error Err(E) back.

### Matching enums:

Rust has built in support for deconstruction/pattern-matching enum types (even ones you define!). 
Example we can do:

    ```
        let x: PixelFmt;
        match x {
            PixelFormat::Yuv{ y, u, v } => {
            println!("Pixel is y: {}, u: {}, v: {}", y, u, v);
            }
            PixelFormat::Rgb{ r, g, b } => {
                println!("Pixel is r: {}, g: {}, b: {}", r, g, b);
            }
            PixelFormat::Greyscale(g) => {
                println!("Greyscale value is: {}", g);
            }
            PixelFormat::Unknown => {
                println!("unknown pix fmt");
            }
        }

    ```
    Rust doesn’t have switch statements that operate over integers like C does, instead it does pattern
    matching this is a very close equivalent that semantically allows for some much more powerful 
    abstractions.

---

## Important enums that are used extensively in programming. 

Here are some of the most common ones:

### 1. Option Enum

The `Option` enum is used to represent a value that may or may not be present. 
It has two variants: `Some` and `None`.

    ```rust
        enum Option<T> {
            Some(T),
            None,
        }
    ```

Example:

    ```rust
        fn divide(x: i32, y: i32) -> Option<i32> {
            if y == 0 {
                None
            } else {
                Some(x / y)
            }
        }

        fn main() {
            match divide(10, 2) {
                Some(result) => println!("Result: {}", result),
                None => println!("Error: Division by zero!"),
            }
        }
    ```

### 2. Result Enum

The `Result` enum is used to represent a value that may be an error. 
It has two variants: `Ok` and `Err`.

    ```rust
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    ```

Example:

    ```rust
        use std::fs::File;

        fn open_file(filename: &str) -> Result<File, std::io::Error> {
            File::open(filename)
        }

        fn main() {
            match open_file("example.txt") {
                Ok(file) => println!("File opened successfully!"),
                Err(error) => println!("Error opening file: {}", error),
            }
        }
    ```

### 3. Ordering Enum

The `Ordering` enum is used to represent the result of a comparison between two values.
It has three variants: `Less`, `Equal`, and `Greater`.

    ```rust
        enum Ordering {
            Less,
            Equal,
            Greater,
        }
    ```

Example:

    ```rust
        fn compare(x: i32, y: i32) -> Ordering {
            if x < y {
                Ordering::Less
            } else if x == y {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }

        fn main() {
            match compare(5, 10) {
                Ordering::Less => println!("5 is less than 10"),
                Ordering::Equal => println!("5 is equal to 10"),
                Ordering::Greater => println!("5 is greater than 10"),
            }
        }
    ```

### 4. Cow Enum

The `Cow` enum is used to represent a value that may be borrowed or owned. 
It has two variants: `Borrowed` and `Owned`.

    ```rust
        enum Cow<'a, B> {
            Borrowed(&'a B),
            Owned(B),
        }
    ```

Example:

    ```rust
        use std::borrow::Cow;

        fn greet(name: Cow<str>) {
            println!("Hello, {}!", name);
        }

        fn main() {
            let name = "John".to_string();
            greet(Cow::Borrowed("Alice"));
            greet(Cow::Owned(name));
        }
    ```

### 5. IpAddr Enum

The `IpAddr` enum is used to represent an IP address. 
It has two variants: `V4` and `V6`.

    ```rust
        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    ```

Example:

    ```rust
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        fn print_ip(ip: IpAddr) {
            match ip {
                IpAddr::V4(ip) => println!("IPv4 address: {}", ip),
                IpAddr::V6(ip) => println!("IPv6 address: {}", ip),
            }
        }

        fn main() {
            print_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            print_ip(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        }
    ```

### 6. SocketAddr Enum

The `SocketAddr` enum is used to represent a socket address. 
It has two variants: `V4` and `V6`.

    ```rust
        enum SocketAddr {
            V4(SocketAddrV4),
            V6(SocketAddrV6),
        }
    ```

Example:

    ```rust
        use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};

        fn print_socket_addr(addr: SocketAddr) {
            match addr {
                SocketAddr::V4(addr) => println!("IPv4 socket address: {}", addr),
                SocketAddr::V6(addr) => println!("IPv6 socket address: {}", addr),
            }
        }

        fn main() {
            print_socket_addr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
            print_socket_addr(SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 
                              8080, 0, 0)));
        }
    ```

Additional enums that are commonly used for systems and embedded rust programming:
---

### 1. Poll Enum

The `Poll` enum is used to represent the result of a non-blocking operation. 
It has two variants: `Ready` and `Pending`.

    ```rust
        enum Poll<T> {
            Ready(T),
            Pending,
        }
    ```

Example:

    ```rust
        use std::io::{Read, Poll, PollEvent, PollFd};

        fn read_non_blocking(fd: &mut PollFd) -> Poll<usize> {
            // ...
        }

        fn main() {
            let mut fd = PollFd::new();
            match read_non_blocking(&mut fd) {
                Poll::Ready(bytes_read) => println!("Bytes read: {}", bytes_read),
                Poll::Pending => println!("Operation pending"),
            }
        }
    ```

### 2. SeekFrom Enum

`SeekFrom` enum is used to represent a seek operation on a file. 
It has three variants: `Start`, `Current`, and `End`.

    ```rust
        enum SeekFrom {
            Start(u64),
            Current(i64),
            End(i64),
        }
    ```

Example:

    ```rust
        use std::io::{Seek, SeekFrom};

        fn seek_file(file: &mut std::fs::File, offset: SeekFrom) -> std::io::Result<()> {
            file.seek(offset)
        }

        fn main() {
            let mut file = std::fs::File::open("example.txt").unwrap();
            seek_file(&mut file, SeekFrom::Start(10)).unwrap();
        }
    ```

### 3. IoPriority Enum

The `IoPriority` enum is used to represent the priority of an I/O operation. 
It has three variants: `Low`, `Normal`, and `High`.

    ```rust
        enum IoPriority { Low,
            Normal,
            High,
        }
    ```

Example:

    ```rust
        use std::io::{IoPriority, IoPriorityExt};

        fn set_io_priority(file: &mut std::fs::File, priority: IoPriority) -> std::io::Result<()> {
            file.set_priority(priority)
        }

        fn main() {
            let mut file = std::fs::File::open("example.txt").unwrap();
            set_io_priority(&mut file, IoPriority::High).unwrap();
        }
    ```

### 4. SockAddr Enum

The `SockAddr` enum is used to represent a socket address. 
It has two variants: `V4` and `V6`.

    ```rust
        enum SockAddr {
            V4(SockAddrV4),
            V6(SockAddrV6),
        }
    ```

Example:

    ```rust
        use std::net::{SockAddr, SockAddrV4, SockAddrV6, Ipv4Addr, Ipv6Addr};

        fn print_sock_addr(addr: SockAddr) {
            match addr {
                SockAddr::V4(addr) => println!("IPv4 socket address: {}", addr),
                SockAddr::V6(addr) => println!("IPv6 socket address: {}", addr),
            }
        }

        fn main() {
            print_sock_addr(SockAddr::V4(SockAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
            print_sock_addr(SockAddr::V6(SockAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0)));
        }
    ```

### 5. SocketType Enum

`SocketType` enum is used to represent the type of a socket. 
It has several variants, including `Stream`, `Datagram`, `SeqPacket`, `Raw`, and `Rdm`.

    ```rust
        enum SocketType {
            Stream,
            Datagram,
            SeqPacket,
            Raw,
            Rdm,
        }
    ```

Example:

    ```rust
        use std::net::{SocketType, TcpStream};

        fn create_socket(socket_type: SocketType) -> std::io::Result<TcpStream> {
            match socket_type {
                SocketType::Stream => TcpStream::connect("example.com:80"),
                _ => Err(std::io::Error::from_raw_os_error(22)),
            }
        }

        fn main() {
            create_socket(SocketType::Stream).unwrap();
        }
    ```

### 6. UnixRights Enum

The `UnixRights` enum is used to represent the rights of a Unix socket. 
It has several variants, including `Read`, `Write`, and `Full`.

    ```rust
        enum UnixRights {
            Read,
            Write,
            Full,
        }
    ```

Example:

    ```rust
        use std::os::unix::net::{UnixRights, UnixStream};

        fn create_unix_socket(rights: UnixRights) -> std::io::Result<UnixStream> {
            match rights {
                UnixRights::Read => UnixStream::connect("/tmp/example.sock"),
                UnixRights::Write => UnixStream::connect("/tmp/example.sock"),
                UnixRights::Full => UnixStream::connect("/tmp/example.sock"),
            }
        }

        fn main() {
            create_unix_socket(UnixRights::Full).unwrap();
        }
    ```

### 7. InterruptPriority Enum

The `InterruptPriority` enum is used to represent the priority of an interrupt. 
It has several variants, including `Low`, `Medium`, and `High`.

    ```rust
        enum InterruptPriority {
            Low,
            Medium,
            High,
        }
    ```

Example:

    ```rust
        use cortex_m::interrupt::{InterruptPriority, Interrupt};

        fn set_interrupt_priority(interrupt: Interrupt, priority: InterruptPriority) {
            match priority {
                InterruptPriority::Low => interrupt.set_priority(0),
                InterruptPriority::Medium => interrupt.set_priority(1),
                InterruptPriority::High => interrupt.set_priority(2),
            }
        }

        fn main() {
            let interrupt = Interrupt::new();
            set_interrupt_priority(interrupt, InterruptPriority::High);
        }
    ```

### 8. PinState Enum

The `PinState` enum is used to represent the state of a pin. 
It has several variants, including `Input`, `Output`, `Alternate`, and `Analog`.

    ```rust
        enum PinState {
            Input,
            Output,
            Alternate,
            Analog,
        }
    ```

Example:

    ```rust
        use stm32f3::gpio::{PinState, Pin};

        fn set_pin_state(pin: Pin, state: PinState) {
            match state {
                PinState::Input => pin.set_input(),
                PinState::Output => pin.set_output(),
                PinState::Alternate => pin.set_alternate(),
                PinState::Analog => pin.set_analog(),
            }
        }

        fn main() {
            let pin = Pin::new();
            set_pin_state(pin, PinState::Output);
        }
    ```

### 9. ClockSource Enum

The `ClockSource` enum is used to represent the source of a clock. 
It has several variants, including `HSE`, `HSI`, `LSE`, and `LSI`.

    ```rust
        enum ClockSource {
            HSE,
            HSI,
            LSE,
            LSI,
        }
    ```

Example:

    ```rust
        use stm32f3::rcc::{ClockSource, RCC};

        fn set_clock_source(rcc: RCC, source: ClockSource) {
            match source {
                ClockSource::HSE => rcc.set_hse(),
                ClockSource::HSI => rcc.set_hsi(),
                ClockSource::LSE => rcc.set_lse(),
                ClockSource::LSI => rcc.set_lsi(),
            }
        }

        fn main() {
            let rcc = RCC::new();
            set_clock_source(rcc, ClockSource::HSE);
        }
    ```

### 10. ResetMode Enum

The `ResetMode` enum is used to represent the mode of a reset. 
It has several variants, including `Software`, `Hardware`, and `Watchdog`.

    ```rust
        enum ResetMode {
            Software,
            Hardware,
            Watchdog,
        }
    ```

Example:

    ```rust
        use stm32f3::rcc::{ResetMode, RCC};

        fn set_reset_mode(rcc: RCC, mode: ResetMode) {
            match mode {
                ResetMode::Software => rcc.set_software_reset(),
                ResetMode::Hardware => rcc.set_hardware_reset(),
                ResetMode::Watchdog => rcc.set_watchdog_reset(),
            }
        }

        fn main() {
            let rcc = RCC::new();
            set_reset_mode(rcc, ResetMode::Software);
        }
    ```
Each enum has its own specific use case and can be used to make your code more 
expressive and efficient.
