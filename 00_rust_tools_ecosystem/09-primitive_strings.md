# Primitive Strings:


## String and memory:

Rust String manage memory automatically:
As String is a wrapper around Vector u8.
When we create a string Rust allocates memory on the heap to strore the string data.
When strings go out of scope their associated memory is automatically de-allocated, 
this helps prevent memory leaks and ensures memory safety.

However when manupulating strings its vital to understand when ownership is transferred 
and when copies are made.

Rust way of handling strings is unique compared to many other languages, with its 
design focused on safety, performance, and memory management.
Things that need to be kept in mind while handling strings:
    Ownership
    Encoding
    string types
  
## String Type: 

### 1. &str ( String Slice )
    * String slice is an immutable reference to a sequence of UTF-8 encoded characters.
    It's a view into a "String" or a literal string.
    * &str is generally used for borrowed string data, meaning it doesn't own the data.
    * It is fixed in size and can point to a part of a larger string.
    
    Ex: 
        ` let greeting: &str = "Hello world"; `
    
    * Usage: commonly used when you want to reference an existing string without 
    needing to own or modify it.
    
### 2. "String":
    * A "String" is an owned, growable string type,Its stored on the heap and allows
    modifications (mutability)
    * A "String" is a UTF-8 encoded, line "&str", but it owns the memory that holds the 
    string data.
    * A "String" is often created from a string literal, and it can be modified:
    Ex:
        let mut greeting: String = String::from("Hello");
        greeting.push_str(", World"); // Modify the string
        
    * Usage: used when you need a dynamically-sized, owned, and mutable string.
    
- UTF-8 Encoding:
    * utf-8 is the default encoding used for string in Rust:
        - A "String" or "&str" in rust represents a sequence of valid utf-8 bytes.
        - Rust strings can hold unicodes. emojis, non-latin ....
    * Each character in a string may occupy 1 to 4 bytes, depending on the Unicode char
    * This flexibility allows rust to support wide range of characters whilt still 
    being memory efficient.

### 3. String Ownership:
    * Rust strings are subject to ownership system. This ensures memory safety and 
    avoids issues like dangling ptrs.
    * A "String" types owns the memory where the string's data is stored on the heap.
    When a "String" is moved, the ownership of the memory is transferred, and the old
    variable is no-longer accessible.
    * String slice (&str) on the other hand, borrows the data and does not own it. 
    This is a reference, and it must adhere to Rust's borrowing rules to ensure safety.
    
### 4. Mutability:
    * "String" is mutable. 
    * &str is immutable
    
### 5. Conversion between "String" and "&str":
    * "String" to "&str": done by borrowing a reference to the "String"
    Ex: 
        let s: String = String::from("hello");
        let s_ref: &str = &s;
        
    * "&str" to String: (involves creating a new "String" instance from the string slice):
    Ex:
        let s: &str = "hello";
        let s_owned: String = s.to_string();
        // or
        let s_owned: String = String::from(s);

### 6. Common Operations:
    * Appending: You can append data to a "String" using methods like "push_str" or "push".
    Ex:
        let mut s = String::from("hello");
        s.push_str(",World!"); // Add a string slice/
        s.push("!"); // Adds  a single character.
        
    * Concatination: using "+" operator we can concatinate "String" and "&str".
    Ex:
        let s1 = String::from("hello");
        let s2 = "World";
        let s3 = s1 + s2;
        
    Alternatively you can use "format!" macro to create a new "String" without taking
    ownership of the inputs:
    Ex:
        let s1 = String::from("hello");
        let s2 = "World";
        let s3 = format!("{}{}",s1,s2); // s1 is not moved here.
        
### 7. performance Considerations:
    * Heap Allocation: A "String" allocated memory on the heap, which allows it to 
    grow dynamically, but this means there's some performance overhead.
    * UTF-8 Encoding: Use of utf-8 is efficient for most languages, but can create 
    complexities when indexing into strings because a character might not correspond
    to a string byte ( i.e: multi-byte characters.)
    
### 8. String indexing:
    * Rust does not allow indexing into strings by byte position. This is because of
    utf-8 encoding, where characters may take more then one byte.
    * To access characters in a string, you must use iterators or methods like 
    "chars()" or "bytes()" to ensure you handle the variable-length encoding
    correctly.
    Ex:
        let s = "hello";
        for c in s.chars() {
            println!("{}",c); //prints each character.
        }
    
### 9. Strings and Memory Safety:
    * Rust's Ownership and Borrowing rules make working with Strings safe.
    Ex:
        - If you try to use a "String" after it has been moved, rust will not allow 
        it and will generate a compile-time error.
        - Similarly Rust ensures that String slice (&str) do not outlive the data
        they refer to ( handled by rust borrowing rules check )
        
### 10. Other String methods:
    * len() return the number of vytes in the string:
    ex:
        let s = "hello, World!";
        println!("Length of s : {}". s.len());
        
    * contains() : checks if a substring exists within a string:
    ex:
        let s = "hello, World!";
        if s.contains("world") {
            println!("Found world");
        }
    * replace() : replace part of the string with another string:
    ex:
        let s = "hello, World!";
        let new_s = s.replace("world!", "Rust!");
        println!("{}",new_s); // hello, Rust!
        
### 10: recap:
    * Rust has 2 primary string types: "String" (owned, mutable) and &str (borrowed, immutable)
    * String: heap allocated
    * &str: string slice are views into string data.
    * Strings in rust are encoded into UTF-8 supports all unicode chars.
    * Ownership and Borrowing rules provides memory safety for strings and prevents
    issues related to dangling ptrs.
    * You can not directly index into a String by position because of UTF-8 encoding,
    but we can use iterators and string methods to work with string safely.
    
### 11. Both of the primitive types String and &str  are part of Rust's "std library".
    And both are preluded into every rust program by default, unless you explicitly 
    opt to out the prelude ( embedded systems )
    
    Ex:
        #![no_std] 
        extern crate alloc;
        
        use alloc::string::String;
        
        fn main() {
            let s: String = String::from("hello world");
            // &str is still implicitly available as its part of rust core lang
        }
        
    - In a #![no_std] environment the "std" lib is not linked, so you will need to 
    rely on other parts of the Rust'e ecosystem, like "alloc" which provides
    String and other heap-allocated types.
    
    - #![no_std] env is generally used when working with low-level env, that does
    not have access to rust's std lib, which is offen the case when working with the 
    embedded systems with limitations on HW.
    In which case we need to rely on external crates: 
        Common Embedded Crates: embedded-hal, cortex-m, nrf52840-hal, stm32f4, etc.,
        are commonly used in no-std environments for controlling hardware.
    
    Ex:
        #![no_std]  // No standard library, we're writing a bare-metal OS
        
        // OS-related functionality
        pub fn kernel_main() -> ! {
            // Kernel setup code (e.g., setting up memory, I/O)
            loop {}
        }
