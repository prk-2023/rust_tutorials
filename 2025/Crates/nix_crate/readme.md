nix crate: Provide friendly binding to various *nix platform APIs. 

For many system APIs it provides a safe alternative to the unsafe APIs that are exposed by the "libc crate".
This is achieved by wrapping the "libc" functionality with "types/abstractions" that enforce legal/safe
usage ( i.e: It wraps the unsafe C-level functions in the `libc` crate with Rust types that enforce safety and handle errors more gracefully. )

Example of 
