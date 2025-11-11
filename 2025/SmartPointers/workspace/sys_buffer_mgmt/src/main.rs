use std::ops::{Deref, DerefMut};

// --- The Smart Pointer: MemoryBuffer ---
struct MemoryBuffer {
    // We use Vec<u8> for safety, but this simulates a pointer to
    // a block of system-allocated memory in a real-world scenario.
    data: Vec<u8>,
    id: u32,
}

// --- 1. Implementing Drop for Cleanup (RAII) ---
// This ensures the resource (the memory block) is explicitly released
// when the MemoryBuffer variable goes out of scope.
impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        // Cleanup code runs here. In a real scenario, this would call
        // a system function like `free(self.data_ptr)`.

        // Simulating the cleanup process:
        println!("🗑️ [DROP] Releasing Memory Buffer ID: {}", self.id);
        println!("🗑️ [DROP] Data cleaned up: {:?}", self.data);
    }
}

// --- 2. Implementing Deref for Read Access (&T) ---
// Allows the smart pointer to be treated as an immutable byte slice (&[u8]).
impl Deref for MemoryBuffer {
    // The target type when dereferencing is a slice of bytes
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        // Return a reference to the inner data as a slice
        &self.data
    }
}

// --- 3. Implementing DerefMut for Write Access (&mut T) ---
// Allows the smart pointer to be treated as a mutable byte slice (&mut [u8]).
impl DerefMut for MemoryBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Return a mutable reference to the inner data as a mutable slice
        &mut self.data
    }
}

// --- Helper Functions simulating system calls ---

// Function expects a regular immutable slice (&[u8])
fn read_config_data(data: &[u8]) {
    println!("   [READ] System read: {:?}", data);
}

// Function expects a regular mutable slice (&mut [u8])
fn write_log_entry(data: &mut [u8], timestamp: u8) {
    if let Some(byte) = data.get_mut(data.len() - 1) {
        *byte = timestamp; // Overwrite the last byte (simulating a change)
    }
    println!("   [WRITE] Log entry written (last byte updated).");
}

fn main() {
    println!("--- Program Start ---");

    // Create the smart pointer, simulating memory allocation
    let mut buffer = MemoryBuffer {
        data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00], // Initial raw data
        id: 101,
    };

    // --- DEMONSTRATION OF DEREF COERCION (READ-ONLY) ---
    // `read_config_data` expects `&[u8]`. We pass `&buffer` (which is `&MemoryBuffer`).
    // Rust automatically calls `buffer.deref()` to coerce it to `&[u8]`.
    read_config_data(&buffer);

    // **Deref Coercion for Method Calls:**
    // `len()` is a method on `&[u8]`, not `MemoryBuffer`.
    // Rust calls `deref()` automatically to resolve the method call.
    println!("   [INFO] Buffer size: {} bytes", buffer.len());

    // --- DEMONSTRATION OF DEREFMUT COERCION (MUTABLE) ---
    // `write_log_entry` expects `&mut [u8]`. We pass `&mut buffer`.
    // Rust automatically calls `buffer.deref_mut()` to coerce it to `&mut [u8]`.
    write_log_entry(&mut buffer, 0x1A); // Write a timestamp (0x1A) to the end

    // Read again to show the change
    read_config_data(&buffer);

    // --- FORCING EARLY DROP (Optional) ---
    // If we needed to release the resource before the end of the scope:
    // std::mem::drop(buffer);
    // println!("   [INFO] Buffer 101 has been manually dropped.");

    println!("--- Program End ---");
    // **AUTOMATIC DROP EXECUTION**
    // The `buffer` variable goes out of scope here.
    // Rust automatically calls `buffer.drop()` (the code we implemented)
    // to ensure the simulated memory is released.
}
