# Rust Ownership:

Rust Ownership is the most Unique feature and has deep implications for the rest of the language:

- Enables Rust to make memory safe guarantees without the needing a garbage collector, ( key is to understand
  how it works. )
- Along with Ownership several other related features also need to be looked to get a higher understanding
  of the working of Rust. 
  Key Topics *Borrowing, Slices, and how Rust lays out in memory*.

Ownership:
- Ownership is basically set of Rules that govern how Rust program managed memory:

    All programs have a way to work with computers memory while running, Some languages use GC ( garbage 
    collection to take care of the cleaning process of memory after use ). While in some languages 
    programmers must explicitly allocate and free memory.

    -> In Rust: Mem is managed through system of Ownership with a set of rules that the compiler checks <- 
    Note if the program violates any of these rules, the program will not pass compilation, forcing used to
    fix errors before they get into the system.

- Languages using GC add latency to run extra checking for garbage collection, where as on the other hand
  Rust Ownership Rules do not slow down the program as there is no additional checking for automatic memory 
  handling to free up unused memory.)

## Recap of Stack and Heap:

Both Stack and Heap are parts of memory available to your program at runtime.
Since Rust is a systems programming language a value is on stack or heap affects how the language behaves
and why the programmer needs to make certain decisions.  

Both stack & heap are part of program's available memory at runtime, but the way they are structured is
different. 

| Feature / Aspect                | **Stack**                                           | **Heap**                                                                       |
| ------------------------------- | --------------------------------------------------- | ------------------------------------------------------------------------------ |
| Memory Organization             | Ordered (Last In, First Out - LIFO)                 | Unordered, dynamic                                                             |
| Data Insertion                  | Pushing onto the stack                              | Allocating on the heap                                                         |
| Data Removal                    | Popping off the stack                               | Deallocating (manually or via garbage collection/ownership)                    |
| Speed                           | Faster                                              | Slower (due to searching and bookkeeping)                                      |
| Access Time                     | Faster (data is close together)                     | Slower (requires pointer dereferencing, data can be scattered)                 |
| Size Requirement                | Must have a **known, fixed size** at compile time   | Can store data with **unknown or dynamic size**                                |
| Storage of Data                 | Directly stores the actual value                    | Stores a *pointer* on the stack, which refers to data in the heap              |
| Use Case Example                | Local variables, function parameters                | Dynamically-sized data like vectors, strings                                   |
| Memory Management               | Automatic via stack pointer                         | Manual or via language features (e.g., ownership in Rust)                      |
| Function Call Behavior          | Arguments and local variables pushed onto the stack | May include pointers to heap data                                              |
| Analogy                         | Stack of plates: Add/remove from the top            | Restaurant table: Host finds a suitable spot, gives you the location (pointer) |
| Processor Efficiency            | High (less jumping in memory)                       | Lower (more jumping in memory)                                                 |
| Connection to Ownership in Rust | Less relevant                                       | Ownership manages heap memory usage, duplication, and cleanup                  |

Part of Ownership is related with stack/heap memory.

### stack:
- Stores values in last in first out manner.
- Pushing or popping data on to stack is only done at the top of the stack.
- To access a middle variable on the stack elements above it must be popped. 
- All data that gets stored on the stack must have a fixed size and is done at compilation time.
- Immutable's go on to stack.
- Pushing on to stack if faster then allocating on to a heap as there is no memory allocator request and the
  stack variables get allocated in the binary as the size of these is fixed. 

### Heap:
- Heap is less organized:
  When you allocate data on the heap, the OS allocator find a sufficiently large free space, and marks it  *in
  use* and returns a pointer to that memory address. *This is call allocating on the heap*. 

  ( Note: Pushing values on to stack is not allocation. Since stack memory is reserved ahead of time for a
  function call, so its not allocated in the same dynamic sense. 
  Pushing values on to stack is not heap allocation: - it's a simple pointer adjustment in a pre-allocated
  stack frame. )

- Allocating space on heap is more work as the allocator must find a big enough space to hold the data and
  then perform bookkeeping to prepare for the next allocation.

- Accessing data on the heap is also slower then over stack as it requires to follow the pointer to get there. 
  I.E: Heap accesses can involve cache misses and pointer chasing, making them slower than stack accesses,
  which tend to be linear and cache-friendly.

- When code calls a function the values passed into the function ( including potentially pointers to the
  data on the heap) and the function local variables get pushed onto the stack. When the function is done
  with its job those values get popped off the stack.
  i.e two things happen:
  1. Function arguments (values passed into the function) are stored on the stack - either directly ( for
     small *Copy* types ) or as stack-based metadata ( for heap-owning types like *String* or *Vec<T>* ).
     
  2. Local variables that are declared inside the function also live on the stack - including the stack
     portion of heap-owning types.

  3. If a variable owns heap data (like a *String* or *Vec<T>* ), only the pointer, length, and capacity (
     for *Vec/String* ) are stored on the stack. The actual data lives on the heap.

  So when we say "values passed into the function get pushed onto the stack", 
  this refers to the function arguments being stored in stack frames. 
  This includes: values passed into the function get pushed onto the stack, this refers to the function 
  arguments being stored in stack frames. 

  - Copy types like i32, bool, char, etc. are copied and pushed directly.
  - Non-Copy types like String, Vec<T>, etc., move their stack parts (like the pointer/len/cap triple) into 
    the new function.
    ==> The heap data itself is not copied or moved unless you explicitly clone it — only the stack metadata 
    (the pointer) is moved.

- When the function returns, its stack frame (including arguments and locals) is popped off the stack.

- Example:
```rust  
fn main() {
    let s = String::from("hello"); // s lives on stack; points to data on heap
    takes_ownership(s);            // s is moved into the function
    // s is no longer usable here
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string is dropped here: heap data is freed
```
1. 's' is a string stores pointer to heap, length and capacity --- On to stack 
2. takes_ownership(s), the stack values of s ( ptr, len, cap) are moved to the functions 'stack frame'
3. When function is done with its job its stack frames are popped and the owned String is dropped. Which
   will freeup the heap memory.

Keeping track of what part of code are using what data on the heap, minimizing the amount of duplication
data on heap, and cleaning up unused data on the heap to make sure you dont run out of space are all
problems that Ownership addresses.

## Rules of Ownership:

- Each value in Rust has an Owner associated with it.
- There can only be one Owener at a time for a value.
- When Owner goes out of scope the values will be dropped.

- Values can be moved, borrowed immutably (&T) or mutably (&mut T), but with strict rules.
- What happends on returns?
  * If a function returns a `String`, ownership of the heap data moves to the caller.
  * No copy occurs unless `.clone()` is used.
- Why Rust is safer without GC?
  Ownership + Lifetime + Borrowing ensure no dangling pointers, double frees, or Memory leaks in safe code.


### Variable scope:

- When a variable "d" comes into scope, it is valid. 
- the variable remains valid until it goes out of scope.

#### String type:

To understand and illustrate the rules of Ownership we need a complex data type such as String, 
String literals are on stored on the stack and they can be pushed and popped on the stack when its scope is
over, they can also be trivially be copied to new one. 
But for data that gets stored on Heap and explore how Rust knows when to clean that data, and the String
type is a great example.

We focus on String that relates to Ownership. ( this aspect also apply to other complex data types that come
from other crates or provided by the program)

##### memory and allocation.

String literals (&str) are immutable ==> fixed length and are stored on stack. 
[ String literal: represented by ""
  let s = "hello"; // hello is string literal of type &'static str'
  Its a string slice (&str) that points to a statically allocated string in memory.
  The lifetime 'static ==> it lives for the entire duration of the program.
  String literals are immutable.
  As their size is known at compile time they are hardcoded into executable.

]
We can build a `String` from string literal using `from` function 

    `let s = String::from("hello");`

`String` of the above type are mutable and they can grow and shrink as they hold their data on the heap.
- As memory must be requested from the memory allocator at runtime.
- We need a way to returning this memory to the allocator when we are done with the `String`. This is where
  Rust takes a new approach ( instead of using GC or manually allocating and freeing ) In Rust memory is
  automatically released when the variable goes out of scope.

  Example:
  ```
  {
      let s = String::from("hello"); // s is valid from this point forward
      // do stuff with s
  }                                  // this scope is now over, and s is no
                                     // longer valid
  ```
  * Rust internally calls `drop` automatically at the end of variable scope i.e at the closing } bracket.
  * This pattern has a profound impact on the way Rust code is written. 
    It may seem simple right now, but the behavior of code can be unexpected in more complicated situations 
    when we want to have multiple variables use the data we’ve allocated on the heap. 

  ```
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
  ```
  * variable 's' is bind into a `String` with value "hello"
  * Next we create a new strong 'ahoy' and assign it to 's'.
  At this point only the ptr of the `String` gets updated to the memory location where 'ahoy' is stored.
  The original string "hello" immediately goes out of scope. And Rust runs `drop` fun on it and its mem is
  freed right away. 

  ```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
  ```
  * `clone` heap data of the `String` gets copied not just stack data but the data in the heap also gets
    copied.

#### stack-only Data:   "Copy"

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the 
stack, as integers are (we’ll talk more about traits in Chapter 10). 

If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, 
making them still valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop`
trait. 

If the type needs something special to happen when the value goes out of scope and we add the `Copy` 
annotation to that type, we’ll get a compile-time error.

As a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires
allocation or is some form of resource can implement `Copy`. 

Here are some of the types that implement Copy:
    * All the integer types, such as u32.
    * The Boolean type, bool, with values true and false.
    *  All the floating-point types, such as f64.
    *  The character type, char.
    *  Tuples, if they only contain types that also implement Copy. 
       For example, (i32, i32) implements Copy, but (i32, String) does not.

### Ownership and Functions:

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. 
Passing a variable to a function will move or copy, just as assignment does. 

Example with some annotations showing where variables go into and out of scope.
```rust 
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

```

#### Return Values and Scope

Returning values can also transfer ownership.
The ownership of a variable follows the same pattern every time: assigning a value to another variable 
moves it. 
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop 
unless ownership of the data has been moved to another variable.

Taking ownership and then returning ownership with every function is a bit tedious. 

```
    fn main() {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{s2}' is {len}.");
    }
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
```
Rust has a feature for using a value without transferring ownership, called `references`.
Which is a pointer that allows us to access the data strored at that address.


### References and Borrowing:

A `reference` is like a pointer in that it’s an address we can follow to access the data stored at that 
address; that data is owned by some other variable.

Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of 
that reference.

The above tuple progran now using reference can be written as below:

```rust 
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}
fn calculate_length(s: &String) -> usize { // s is the reference to a `String`
    s.len()
} // At this point s goes out of scope , Since s is reference and does not own Ownership of what it owns to 
  // and thus `String` is not dropped.
```
* tuple code in the variable declaration and the function return value is gone
* &s1 is passed to calculate_length and, in its definition, we take `&String` rather than `String`. 
* These ampersands `&` represent references, and they allow you to refer to some value without taking 
  ownership of it. 
* Opposite of referencing by using & is dereferencing, which is accomplished with dereference operator, `*`

* The &s1 syntax lets us create a `reference` that refers to the value of s1 but does not own it. 
  Because the reference does not own it, the value it points to will not be dropped when the reference 
  stops being used.

i.e The scope in which the variable `s` is valid is the same as any function parameter’s scope, but the 
value pointed to by the reference is not dropped when `s` stops being used, because `s` doesn’t have ownership. 

NOTE:  When functions have `references` as iarguments instead of the actual values, we won’t need to return 
the values in order to give back ownership, because we never had ownership.

References can be created to immutable and mutable values. 

#### Borrowing: 

The action of creating a reference is called `borrowing`.

Functions that have `immutable references` can not modify the variables.
Functions that have `mutable references` can modify the variables.
```rust 
    fn main() {
        let mut s = String::from("hello");
        change(&mut s);
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
```

- `mutable references` have one restriction: 
    
    Core rules of Rust’s borrowing system, and it’s essential for ensuring memory safety without a GC.
    This rule prevents data races at compile time, by ensuring that:
        1. No two parts of the code can mutate the same data at the same time.
        2. No one can read the data while it's being mutated.
    In other words: exclusive access for mutation.

##### Borrowing Rules:

1. You can have only one mutable reference `&mut T`
2. You can have any number of immutable references &T, but not both at the same time.
Example:
```
let mut x = 5;
let r1 = &mut x;
let r2 = &x; // ❌ ERROR: cannot borrow `x` as immutable because it is also borrowed as mutable

// the correct form should be 
let mut x = 5;
{
    let r1 = &mut x; // exclusive mutable borrow
    *r1 += 1;
} // r1 goes out of scope here
let r2 = &x; // now we can borrow immutably
```
Rust’s borrow checker tracks the lifetimes and scopes of references to enforce this rule at compile time 
— meaning you’ll never encounter a dangling pointer, data race, or double free in safe Rust.


```
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1}, {r2}");
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:3:14

errori: this code is invalid because we cannot borrow `s` as mutable more than once at a time. 
The first mutable borrow is in r1 and must last until it’s used in the println!, 
but between the creation of that mutable reference and its usage, we tried to create another mutable 
reference in r2 that borrows the same data as r1. 

The benefit of having this restriction is that Rust can prevent data races at compile time. 

A data race is similar to a race condition and happens when these three behaviors occur:

    * Two or more pointers access the same data at the same time.
    * At least one of the pointers is being used to write to the data.
    * There’s no mechanism being used to synchronize access to the data.

- scope:
    We can use { .. } to create a new scope, allowing for multiple mutable references, just not 
    simultaneous ones:
```
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```
Rust enforces a similar rule for combining  mutable and immutable references.
The below code results in error:
```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
We also cannot have a `mutable reference` while we have an `immutable` one to the same value.

Users of an immutable reference don’t expect the value to suddenly change out from under them! 
However, multiple immutable references are allowed because no one who is just reading the data has the 
ability to affect anyone else’s reading of the data.

Note that a reference’s scope starts from where it is introduced and continues through the last time that 
reference is used.
ex:
```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
```
code will compile because the last usage of the immutable references is in the println!, 
before the mutable reference is introduced.

#### Dangling References:
In Rust, the compiler guarantees that references will never be dangling references: 
If we have a reference to some data, the compiler will ensure that the data will not go out of scope before
the reference to the data does.

ex: try to make dangling reference:
```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {            //returns a ref to String
    let s = String::from("hello");  // is the owner of the string 
    &s                              // return reference to s 
}                                   // s goes out of scope and is dropped. so its mem goes away .. Danger
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
NOTE: More on lifetimes later.

To fix this instead of returning the reference we should modify the to return `String` ( which transfers the
ownership of the String)
    ``` 
    fn no_dangle() -> String {
        let s = String::from("hello");
        s 
    }
    ```

### Rules of References:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References should always be valid.



### Slice:

`Slices` let you reference a contiguous sequence of elements in a `collection`.  
`Slice` is a kind of reference, so it does not have ownership.

Note:
[
    - Rust `std` library includes many types of numbers  of very useful data structures called collections.
    - Most other data types represent one specific value, but collections can contain multiple values. 
    - Unlike the built-in array and tuple types, the data that these collections point to is stored on the 
      heap, which means the amount of data does not need to be known at compile time and can grow or shrink
      as the program runs. 
    - Each kind of collection has different capabilities and costs, and choosing an appropriate one for 
      your current situation is a skill you’ll develop over time. 

This article we’ll discuss three collections that are used very often in Rust programs:

- A `vector` allows you to store a variable number of values next to each other.
- A `String` is a collection of characters. 
- A hash map allows you to associate a value with a specific key. 
  It’s a particular implementation of the more general data structure called a map.

]



