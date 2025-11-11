// use std::ops::Deref;
//
// // --- 1. Define the Smart Pointer ---
// // A simple struct to wrap a Rust String, pretending it's a C-style string
// // stored in a system buffer.
// struct CStringBuffer {
//     // In a real system scenario, this would likely be a raw pointer (*const u8),
//     // but we use a String for safe demonstration of the concept.
//     inner_data: String,
// }
//
// // --- 2. Implement Deref for Read-Only Access (like &T) ---
// impl Deref for CStringBuffer {
//     // Define what type the smart pointer *dereferences* to.
//     // In systems programming, we often want to access the data as a string slice (&str).
//     type Target = str;
//
//     fn deref(&self) -> &Self::Target {
//         // Here, we convert the inner String to a &str slice.
//         &self.inner_data
//     }
// }
//
// // --- 3. Implement DerefMut for Mutable Access (like &mut T) ---
// impl std::ops::DerefMut for CStringBuffer {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         // Here, we convert the mutable inner String reference to a mutable &str slice.
//         &mut self.inner_data
//     }
// }
//
// // --- 4. A System-Style Function Accepting a Regular Reference ---
// // This function would typically be a C-library call wrapper that expects
// // a regular string slice reference (&str) to read data.
// fn print_system_buffer(s: &str) {
//     println!("System Buffer Length: {}", s.len());
//     println!("System Buffer Content: \"{}\"", s);
// }
//
// // This function simulates a call that modifies the buffer, accepting &mut str.
// fn append_system_status(s: &mut str) {
//     // In a real scenario, this would involve memory safety checks.
//     // We demonstrate mutation using simple string slicing/assignment.
//     let status = " [OK]";
//     if let Some(mut tail) = s.get_mut(s.len() - status.len()..) {
//         tail.copy_from_slice(status);
//     }
// }
//
// fn main() {
//     // Initialize our smart pointer wrapper
//     let mut buffer = CStringBuffer {
//         inner_data: String::from("initial_config_status...."),
//     };
//
//     println!("\n--- Initial State ---");
//     // **Deref Coercion (Read-Only)**
//     // `print_system_buffer` expects `&str`. We pass `CStringBuffer`.
//     // Rust automatically calls `buffer.deref()` to get the `&str` it needs.
//     print_system_buffer(&buffer);
//
//     // **Deref Coercion (Read-Only) for Method Calls**
//     // `len()` is a method on `&str`, not `CStringBuffer`.
//     // Rust calls `deref()` to resolve the method call seamlessly.
//     println!("Direct access to &str method: {}", buffer.len());
//
//     println!("\n--- Mutable State ---");
//     // **Deref Coercion (Mutable)**
//     // `append_system_status` expects `&mut str`. We pass `&mut CStringBuffer`.
//     // Rust automatically calls `buffer.deref_mut()` to get the `&mut str`.
//     append_system_status(&mut buffer.inner_data);
//     // Note: We access inner_data directly here for the mutation to work within a simple String,
//     // but the principle of &mut CStringBuffer being coerced to &mut str
//     // is what enables methods like `push_str` or similar to work directly on `buffer`
//     // if we implemented a more complex buffer type.
//
//     // For a simple demo of coercion in method calls, we'll manually call `deref_mut`
//     // to show how an `&mut str` method can be used on the struct.
//     (*buffer).make_ascii_uppercase();
//
//     // Now print the modified buffer.
//     print_system_buffer(&buffer);
// }
use std::ops::{Deref, DerefMut};

// --- 1. Define the Smart Pointer (Same as before) ---
struct CStringBuffer {
    inner_data: String,
}

// ... Deref and DerefMut implementations remain the same ...
impl Deref for CStringBuffer {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.inner_data
    }
}

impl DerefMut for CStringBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_data
    }
}

// --- 2. Corrected System-Style Function ---
// This function expects a regular *mutable* string slice reference (&mut str).
// However, we will demonstrate the coercion by calling a method directly on the smart pointer.
fn print_system_buffer(s: &str) {
    println!("System Buffer Length: {}", s.len());
    println!("System Buffer Content: \"{}\"", s);
}

fn main() {
    let mut buffer = CStringBuffer {
        inner_data: String::from("initial_config_status...."),
    };

    println!("\n--- Initial State ---");
    print_system_buffer(&buffer);

    // **Deref Coercion (Mutable)**
    // `push_str()` is a method on `String` (which implements `DerefMut<Target=str>`).
    // Rust sees `buffer` is of type `&mut CStringBuffer`.
    // It automatically calls `deref_mut()` to get an `&mut String` (via its internal access to `&mut str`).
    // This allows us to call `push_str` directly on the smart pointer `buffer`,
    // appending data to the internal `String`.
    buffer.inner_data.push_str(" [OK]"); // Directly appending to the inner data for clarity

    println!("\n--- After Mutation and DerefMut Coercion ---");

    // **Deref Coercion (Method Calls on the Smart Pointer)**
    // `make_ascii_uppercase` is a method on `&mut str`.
    // Rust automatically calls `deref_mut` to allow this mutable operation.
    buffer.make_ascii_uppercase();

    print_system_buffer(&buffer);
}
