# **Rust Ownership & Borrowing: Complete Guide to Memory Management**

## **Table of Contents**
1. [Core Concepts Overview](#core-concepts-overview)
2. [Copy vs Move Semantics](#copy-vs-move-semantics)
3. [Borrowing & References](#borrowing--references)
4. [Practical Examples & Demos](#practical-examples--demos)
5. [Common Type Classifications](#common-type-classifications)
6. [Quick Reference Rules](#quick-reference-rules)
7. [Practice Exercises](#practice-exercises)

---

## **Core Concepts Overview**

### **The Three Key Rules**
1. **Each value has a single owner**
2. **Values are moved or copied on assignment**
3. **References allow temporary access without ownership transfer**

### **Memory Safety Guarantees**
- **Compile-time checks** prevent use-after-move errors
- **No runtime panics** for ownership violations (compilation fails instead)
- **Automatic memory management** without garbage collection

---

## **Copy vs Move Semantics**

### **What Implements Copy?**
```rust
// Primitive types and simple aggregates
let x: i32 = 5;        // Copy
let y: bool = true;    // Copy  
let z: char = 'a';     // Copy
let t: (i32, f64) = (1, 2.0);  // Copy (all elements are Copy)
```

### **What Gets Moved?**
```rust
// Heap-allocated or complex types
let s: String = String::from("hello");  // Move
let v: Vec<i32> = vec![1, 2, 3];       // Move
let b: Box<i32> = Box::new(5);         // Move
```

### **Visualizing the Difference**
```
COPY TYPES (i32, f64, etc.):
let a = 5;       Stack: [a: 5]
let b = a;       Stack: [a: 5, b: 5]  ← Both independent copies

MOVE TYPES (String, Vec, etc.):
let s = String::from("hi");  Stack: [s→"hi"]
let t = s;                   Stack: [t→"hi"]  ← s becomes invalid
```

---

## **Borrowing & References**

### **Immutable Borrowing (`&`)**
```rust
let owner = String::from("I own this");
let borrower1 = &owner;     // Create reference
let borrower2 = &owner;     // Multiple borrows OK
let borrower3 = &owner;     // As many as needed

println!("{}", owner);      // ✅ Original still usable
println!("{}", borrower1);  // ✅ References work
println!("{}", borrower2);  // ✅ All references valid
```

### **Borrowing Never Moves Ownership**
```rust
fn main() {
    let original = String::from("important data");
    
    // Borrowing creates references, doesn't affect ownership
    let reference = &original;
    let another_ref = &original;
    
    // Original still fully owns the data
    let still_owner = original;  // ✅ This moves ownership NOW
    // let bad = reference;      // ❌ Would try to use moved value
}
```

### **Function Parameter Patterns**
```rust
// TAKES ownership (moves)
fn consume_string(s: String) {
    println!("I now own: {}", s);
} // s dropped here

// BORROWS temporarily (no move)  
fn borrow_string(s: &String) {
    println!("I'm borrowing: {}", s);
} // s still owned by caller

// Usage:
let data = String::from("hello");
borrow_string(&data);    // ✅ data still usable after
consume_string(data);    // ❌ data no longer usable after this
```

---

## **Practical Examples & Demos**

### **Complete Working Example**
```rust
#[derive(Debug)]
struct Data {
    value: i32,
}

fn main() {
    demonstrate_copy_types();
    demonstrate_move_types(); 
    demonstrate_borrowing();
    demonstrate_functions();
}

fn demonstrate_copy_types() {
    println!("=== COPY TYPES ===");
    let a = 42;
    let b = a;                    // Copy
    let c = b;                    // Copy  
    println!("a={}, b={}, c={}", a, b, c); // All valid
    
    modify_copy(a);               // Copy passed to function
    println!("a after function: {}", a); // Still valid
}

fn demonstrate_move_types() {
    println!("\n=== MOVE TYPES ===");
    let s1 = String::from("hello");
    let s2 = s1;                  // Move
    // println!("{}", s1);        // ❌ COMPILE ERROR: moved
    println!("s2: {}", s2);       // ✅ s2 now owns it
    
    let v1 = vec![1, 2, 3];
    take_ownership(v1);           // Move to function
    // println!("{:?}", v1);      // ❌ COMPILE ERROR: moved
}

fn demonstrate_borrowing() {
    println!("\n=== BORROWING ===");
    let owner = String::from("I'm the owner");
    
    let ref1 = &owner;            // Borrow
    let ref2 = &owner;            // Borrow again
    let ref3 = &owner;            // And again
    
    println!("Owner: {}", owner);     // ✅ Still owns
    println!("Ref1: {}", ref1);       // ✅ All references
    println!("Ref2: {}", ref2);       // ✅ work fine
    println!("Ref3: {}", ref3);       // ✅
    
    borrow_value(&owner);         // Borrow to function
    println!("Owner still works: {}", owner); // ✅
}

fn demonstrate_functions() {
    println!("\n=== FUNCTION INTERACTIONS ===");
    let num = 100;                    // Copy type
    let text = String::from("text");  // Move type
    
    use_copy_type(num);               // num copied
    println!("num still: {}", num);   // ✅ Still usable
    
    use_move_type(text);              // text moved
    // println!("{}", text);          // ❌ No longer usable
    
    let another_text = String::from("another");
    borrow_only(&another_text);       // borrowed
    println!("still own: {}", another_text); // ✅ Still usable
}

// Helper functions
fn modify_copy(x: i32) {
    println!("Copy parameter: {}", x);
}

fn take_ownership(v: Vec<i32>) {
    println!("I took ownership of: {:?}", v);
}

fn borrow_value(s: &String) {
    println!("Borrowed: {}", s);
}

fn use_copy_type(n: i32) {
    println!("Using copy: {}", n);
}

fn use_move_type(s: String) {
    println!("Taking ownership of: {}", s);
}

fn borrow_only(s: &String) {
    println!("Just borrowing: {}", s);
}
```

---

## **Common Type Classifications**

### **Always Copy Types**
```rust
// Primitive Types
bool, char,
i8, i16, i32, i64, i128, isize,
u8, u16, u32, u64, u128, usize,  
f32, f64,

// Tuples of Copy types
(i32, f64), (bool, char, u8),

// Arrays of Copy types
[i32; 5], [f64; 10],

// Custom types with #[derive(Copy, Clone)]
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }
```

### **Always Move Types**
```rust
// Heap-allocated types
String, Vec<T>, Box<T>, Rc<T>, Arc<T>,

// File and I/O types  
File, TcpStream, MutexGuard<T>,

// Tuples containing move types
(String, i32), (Vec<i32>, bool),

// Custom types without Copy
struct Data { values: Vec<String> }
```

---

## **Quick Reference Rules**

### **Assignment Behavior**
| Operation | Copy Types | Move Types |
|-----------|------------|------------|
| `let b = a;` | Copies value | Moves ownership |
| Original usable after? | ✅ Yes | ❌ No |
| Memory impact | Stack duplication | Ownership transfer |

### **Borrowing Behavior**  
| Operation | Copy Types | Move Types |
|-----------|------------|------------|
| `let ref = &a;` | Creates reference | Creates reference |
| Original usable after? | ✅ Yes | ✅ Yes |
| Can have multiple? | ✅ Yes | ✅ Yes |

### **Function Calls**
| Parameter Type | Copy Types | Move Types |
|----------------|------------|------------|
| `func(value)` | Value copied | Value moved |
| `func(&value)` | Reference borrowed | Reference borrowed |
| Caller keeps original? | ✅ Always | ❌ Unless borrowed |

---

## **Practice Exercises**

### **Exercise 1: Predict the Output**
```rust
fn main() {
    let x = 5;
    let y = x;
    let z = y;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    let s = String::from("hello");
    let t = s;
    // println!("s: {}", s);
    println!("t: {}", t);
}
```

### **Exercise 2: Fix the Errors**
```rust
fn main() {
    let data = String::from("important");
    process_data(data);
    println!("Data: {}", data); // ❌ Fix this
    
    let numbers = vec![1, 2, 3];
    print_numbers(numbers);
    print_numbers(numbers); // ❌ Fix this
}

fn process_data(s: String) { /* ... */ }
fn print_numbers(v: Vec<i32>) { /* ... */ }
```

### **Exercise 3: Implement Borrowing**
```rust
// Convert these functions to use borrowing instead of ownership

// ORIGINAL (moves ownership):
fn get_length(s: String) -> usize {
    s.len()
}

// FIXED (borrows instead):
fn get_length_fixed(/* your code here */) -> usize {
    // implementation
}
```

### **Exercise Solutions**

**Exercise 1 Solution:**
```rust
fn main() {
    let x = 5;        // i32 - Copy
    let y = x;        // Copy
    let z = y;        // Copy  
    println!("x: {}, y: {}, z: {}", x, y, z); // All work: 5, 5, 5
    
    let s = String::from("hello"); // String - Move
    let t = s;                     // Move
    // println!("s: {}", s);       // ❌ s was moved
    println!("t: {}", t);          // ✅ t owns it now: "hello"
}
```

**Exercise 2 Solution:**
```rust
fn main() {
    let data = String::from("important");
    process_data(&data); // Borrow instead of move
    println!("Data: {}", data); // ✅ Now works
    
    let numbers = vec![1, 2, 3];
    print_numbers(&numbers); // Borrow
    print_numbers(&numbers); // ✅ Can borrow multiple times
}

fn process_data(s: &String) { /* ... */ }        // Take reference
fn print_numbers(v: &Vec<i32>) { /* ... */ }     // Take reference
```

**Exercise 3 Solution:**
```rust
// Borrowing version
fn get_length_fixed(s: &String) -> usize {
    s.len()
} // s not dropped - still owned by caller

// Usage:
fn main() {
    let text = String::from("hello world");
    let len = get_length_fixed(&text);
    println!("Length: {}, Text: {}", len, text); // ✅ Both work
}
```

---

## **Summary Cheat Sheet**

```rust
// ✅ COPY: Simple, stack-only data
let a = 42; let b = a; // Both usable

// ❌ MOVE: Complex, heap-allocated data  
let s = String::from("hi"); let t = s; // Only t usable

// ✅ BORROW: Create references to any type
let owner = String::from("data");
let borrower = &owner; // Both usable

// ✅ MULTIPLE BORROWS: Many immutable references
let ref1 = &owner; let ref2 = &owner; // All work

// ❌ USE AFTER MOVE: Compile-time error
let moved = owner; // owner moved
// println!("{}", owner); // ❌ Won't compile
```

This document provides a comprehensive reference for Rust's ownership system - the foundation of Rust's memory safety guarantees without garbage collection.
