# Rust prelude:

The prelude is a set of modules that are automatically imported into every Rust program, making their
contents available without the need for explicit imports using the `use` keyword.


The prelude includes a number of fundamental types and traits, such as:

* `Option`
* `Result`
* `Vec`
* `String`
* `HashMap`
* `HashSet`
* `Box`
* `Rc`
* `Arc`
* `std::fmt::Display`
* `std::fmt::Debug`
* `std::cmp::PartialEq`
* `std::cmp::Eq`
* `std::hash::Hash`
* `std::iter::Iterator`
* `std::iter::IntoIterator`

These types and traits are considered essential to the Rust language and are used extensively throughout
the standard library.

By including them in the prelude, Rust makes it easy to write programs without having to worry about
importing them explicitly.

Complete list of modules and types included in the prelude in the Rust documentation:
[The Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html).

Note that while the prelude includes many useful types and traits, it's still possible to import
additional modules and types using the `use` keyword if needed.

When programming for embedded Rust, it's common to exclude the prelude to reduce the size of the generated
binary and improve performance.

By default, the Rust compiler includes the prelude in every program.
However, you can control this behavior using the `no_std` attribute.

To exclude the prelude, you can add the following attribute to your crate's `lib.rs` file:
```rust
#![no_std]
```
This attribute tells the Rust compiler not to include the prelude in your program.

When you exclude the prelude, you'll need to import the types and traits you need explicitly using the
`use` keyword. For example:


```rust
use core::option::Option;
use core::result::Result;
```
Note that the `core` library is a subset of the `std` library that's designed for use in embedded systems.
It includes many of the same types and traits as the `std` library, but without the overhead of the `std`
library's dependencies.

Alternatively, you can use the `#![no_std]` attribute in combination with the
`#![feature(core_intrinsics)]` attribute to include only the core intrinsics, which are a set of
low-level functions that provide basic operations like arithmetic and memory management.

To include the prelude, you can simply omit the `#![no_std]` attribute.
However, keep in mind that this will include the entire `std` library, which may not be suitable for
embedded systems.

Here's an example of how you might structure your `lib.rs` file to exclude the prelude:
```rust
#![no_std]

use core::option::Option;
use core::result::Result;

// Your code here
```
And here's an example of how you might structure your `lib.rs` file to include the prelude:
```rust
// No attributes needed

// Your code here
```
Note that the `#![no_std]` attribute only affects the current crate, so if you're using a library that
includes the prelude, you'll still need to import the types and traits you need explicitly.

In summary:

* To exclude the prelude, use the `#![no_std]` attribute and import types and traits explicitly.
* To include the prelude, omit the `#![no_std]` attribute.
* Use the `core` library to access types and traits that are similar to those in the `std` library, but
  without the overhead of the `std` library's dependencies.

---
**Programs Packed with Rust binary**
---

When a Rust program is compiled, several other programs are included in the binary:

1.  **`std`**: 
    The Rust standard library is included in the binary. 
    However, the `std` library is designed to be highly modular, so only the parts of the library that 
    are actually used by the program are included in the binary.

2.  **`libc`**: 
    The C standard library is included in the binary, as Rust uses it for certain operations such as file 
    I/O and networking.

3.  **`libm`**: 
    The math library is included in the binary, as Rust uses it for certain mathematical operations.

4.  **`libgcc`**: 
    The GCC runtime library is included in the binary, as Rust uses it for certain operations such as 
    exception handling.

5.  **`libpthread`**: 
    The POSIX threads library is included in the binary, as Rust uses it for certain operations such as 
    threading.

6.  **`libdl`**: 
    The dynamic linker library is included in the binary, as Rust uses it for certain operations such as 
    dynamic linking.

7.  **`librt`**: The real-time library is included in the binary, as Rust uses it for certain operations 
    such as timing and scheduling.


