# Ownership, Reference, Borrowing, Slicing:
---

Meta: Before learning about Ownership there are 2 important aspects to note:

1. Rust aims at speed and security and it achives this with "zero-cost abstractions". 
Zero Cost Abstractions: Rust abstractions cost as little as possible in order to make them work.
Ownership is a zero-cost anstraction as all the analysis is done at Compilation time and does not show in
the run-time cost of these features.

2. sharp learning curve: this is the result of programmers mental model does not match the actual rules set
   by Rust programming language. 

## Ownership:

Ownership in Rust refers to the relationship between a value and the variable that owns it.
The Owner of the value is responsible for allocation and deallocating the value when its not required.

The allocation of the value happens on stack or heap memory which depends on their type and  how they are 
used in the program.

Stack Allocated Values:

    Values that have fixed sizes and known at compilation time, example Integer, Boolean, tuple... (basic
    primitive data types). 
    These values are allocated on the stack and automatically de-allocated when they go out of scope.

Heap Allocated Values:

    These values are those that have a dynamic size, such as strings type, vectors, and boxes. 
    These values are allocated on the heap using dynamic memory allocation, and are deallocated when the 
    value is dropped. 
    Note: the dynamic memory allocation is done by the related memory calls that in turn gets it from OS.

In both type the concept of ownership applies and the owner of the value is responsible for ensuring that
it is properly deallocated when its no longer needed.
- Ownership is the fundamental concept of Rust. They are sets of rules that govern how Rust program manages
  memory.

- The rules have deep implications for the rust language.

- Ownership gives memory safety guarantees without the need for garbage collectors.

- Borrowing, reference slices are related to ownership and are required to understand how rust operates.

- Ownership helps prevent common programming erros like null ptr dereferences, data race conditions.

- Rust Ownership deals with managing the memory and lifetime of values. 

- Rust has a different approch for managing memory through a system of ownership with a set of rules that
  the compiler checks.

  If Any of these rules get violated then the program will not compile.

  This concept of ownership is a new compared with the current systems programming languages. And
  understanding the concept of ownership is mandatory for developing code taht is safe and efficient.

- In Rust, every value has an owner that is responsible for managing its memory. 
  When a value is created, it is is assigned to a variable, which becomes its owner. 

  The owner is responsible for ensuring that the value is valid and accessible for as long as it's needed.

- **Rules of Ownership**:  There are three rules of ownership in Rust:
    1. **Each value in Rust has an owner.**
    2. **There can only be one owner at a time.**
    3. **When the owner goes out of scope, the value will be dropped.**

- Ownership is the fundamental concept of Rust. 
- The rules that govern how Rust program manages memory.
- Ownership gives memory safety guarantees without the need for garbage collectors.
- Borrowing, reference slices are related to ownership and proper understanding is required for programming
  in rust.
  - Ownership helps prevent common programming erros like null ptr dereferences, data race conditions.
  - Ownership also deals with managing the memory and lifetime of values.
  - The Rust ownership rules are checked by the compiler specifically by the borrow checker.


### **Borrow Checker**: 
    Its a part of Rust compiler that checks the code at compilation time and enforces
        the code(variables ) satisfies the ownership rules. 
        Borrow checker checks:
            - Ownership: verify each value has only a single owner, and owner is responsible for
              deallocating the value when its no longer needed.
            - Borrowing: make sure the borrowing is done correctly, and the borrowing rules are followed.
              For example it checks if the value is not borrowed as mutable and immutable at same time 
            - Lifetime: The borrow checker ensures that the lifetimes of values are correctly managed, and
              that values are not used after they have gone out of scope.

        Borrow checker is a key feature of the Rust language, and it helps to prevent common errors such 
        as null pointer dereferences, use-after-free bugs, and data corruption.

    - In addition to borrow checker rust also has a rumtime component called "drop system" that is
      responsible for deallocating values that go out of scope ( the DropSystem only drops those variables
      that are properly managed by the borrow checker)
    - If the borrow checker finds a error the program will not be compiled.
    - The rules have deep implications for the rust language.

### Ownership Examples:

- **Example 1: Simple Ownership**
    
    In Rust, string literals are of type " &str " , which is a reference to a string slice. 
    When you assign a string literal to a variable, the variable is a reference to the string literal, 
    not the owner of the string literal.
    
    No transfer of owenship occurs for the below code:
    Because string literals are stored in the program's binary and are not allocated on the heap.
    ```rust
        let s = "I am String literal"; // s is the owner of the string "hello"
    ```

- **Example 2: Ownership Transfer**
    ```rust
        let s = String::from("hello"); // s is the owner of the string "hello"
        let t = s; // t takes ownership of the string "hello", s is no longer the owner
    ```
    In this case string 's' gets heap allocation and it gets ownership.

    `s` is initially the owner of the string "hello". 

    When we assign `s` to `t`, `t` takes ownership of the string "hello", and `s` is no longer the owner.
    accessing s will break the compilation with a Error: [E0382] and message " value borrowed "

- **Example 3: Borrowing**
    ```rust
        let s = String::from ("hello"); // s is the owner of the string "hello"
        let len = calculate_length(&s); // s is borrowed, but still owns the string
    ```
    `s` is the owner of the string "hello". We pass a reference to `s` to the `calculate_length` function,
    which borrows `s` but does not take ownership. 

    `s` still owns the string "hello" after the function call.

- **Types of Ownership**

- Rust has two types of ownership:

1. **Move**: When ownership is transferred from one variable to another, it's called a move. 
    In Move operation the original owner can no longer use the value.

2. **Borrow**: When a reference to a value is passed to a function or assigned to another variable, it
   called a borrow. The original owner still owns the value.

**Example 4: Move vs Borrow**
    ```rust
        let s = "hello".to_string(); // s is the owner of the string "hello"
        let t = s; // move: s no longer owns the string "hello"
        let len = calculate_length(&s); // error: s no longer owns the string "hello"

        let s = "hello".to_string(); // s is the owner of the string "hello"
        let len = calculate_length(&s); // borrow: s still owns the string "hello"
    ```
    In the first example, `s` is moved to `t`, so `s` no longer owns the string "hello". 
    In the second example, `s` is borrowed by the `calculate_length` function, but `s` still owns the str.

## memory:

- Mem safety: Its a property of a program where every memory pts used always point to a valid memory.
    i.e allocated and of the correct type/size.

- mem safety is a correctness issue: 
    A memory unsafe program may crash or produce non-deterministic output depending on the bug.

  - There are many languages that allow us to write "memory unsafe" code in the sense that it's fairly easy
    to understand bugs. Ex:
    - Dangling pointer: pts that point to invalid data ( this will be more clear when we look at how data is
      stored in memory). (https://stackoverflow.com/questions/17997228/what-is-a-dangling-pointer)
    - Double free: trying to free that same memory location more then once, this can leade to
      "undeterministic behaviour".(https://stackoverflow.com/questions/21057393/what-does-double-free-mean))
    - Unlike languages that come with GC, Rust uses the concept of owenship, Borrowing to handle issues
      related to memory.

    - So when we say rust comes with memory safety, we refer to the fact that by default Rust compiler does
      not allow us to wire core that is not memory safe. 

## Stack and Heap:

- Stack/Heap are both parts of the programs memory but they both are represented in different structures.

- Stack values have fixed size and are stored in LIFO order, accessing stack variables is fast.
- Adding data onto stack is called "pusing on to stack".
- Removing data from stack is called "popping off the stack".

- Heap: Data with an unknown size at compilation time or a size that might change over time must be stored
  on the Heap.

- Heap is less origanized then stack, when a variable is supposed to store on a stack, you request a certain
  amount of space. The memory allocator finds an empty spot in the heap that is bigh enough, marks it as
  being in use and reuturns a pointer, which is the address of that location. This process is called
  allocating on the heap or just 'allocating'. 
  ( pusing values onto stack is not called pushing )

- The pts to the heap is known and fixed size, you can store the ptr on the stack, but the actual data is
  stored on heap.

- Pushing to the stack is faster then allocating on the heap beacause they allocator never has to search for
  a place to store new value. And the location is always at the to of the stack.
  Comparitively allocating space on the heap requires more work and also perform bookkeeping to prepare for
  the next allocation.

- Access data in the heap is slower then stack as its required to follow the ptr to get there.

- when your code calls a function, the values paseed into the function ( including, potentially, pts to the
  data on the heap) and the functions local variables get pused onto the stack. 
  when the function is done with its task those values get popped off the stack.

- So what goes into heap and stack depends on the type of data we are dealing with.

## Ownership Rules and memory:

 - Each value in rust has a owner.
 - there can only be one single owner at a given time.
 - when ower goes out of scope the values get dropped. 

- Variable scope: similar with other programming languges.
    - When the program leaves a block in which a variable is declared, that variable will be dropped,
      dropping its values with it.
    - The block could be a function an if statement or pretty much anything that introduces a new code with
      curly braces. 
- When a variable goes out of scope rust internally calles a "drop()" function, which gets called at the end
  of curly braces or at the end of scope of a variable.

- Example:
    ```
        let names = vec!["Pascal".to_string(), "Christoph".to_string()];
    ```
This creates a vector of names. A vector in Rust is like an array, or list, but it’s dynamic in size. 
We can push() values into it at run-time. Our memory will look something like this:


```
            [–– names ––]
            +–––+–––+–––+
stack frame │ • │ 3 │ 2 │
            +–│–+–––+–––+
              │
            [–│–– 0 –––] [–––– 1 ––––]
            +–V–+–––+–––+–––+––––+–––+–––+–––+
       heap │ • │ 8 │ 6 │ • │ 12 │ 9 │       │
            +–│–+–––+–––+–│–+––––+–––+–––+–––+
              │\   \   \  │
              │ \   \    length
              │  \    capacity
              │    buffer │
              │           │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
            │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+
                          │
                          │
                        +–V–+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
                        │ C │ h │ r │ i │ s │ t │ o │ p │ h │   │   │   │
                        +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
```
- Notice how the vector object itself, similar to the string object earlier, is stored on the stack with its
  capicity, and length. It also comes with a ptr pointing at the location in the heap where the vector data
  is located. The string object of the vector are then stored on the heap, which in turn own their dedicated
  buffer.

- This creates a tree structure of data where every value is owned by a single variable. When names goes 
  out of scope, its values will be dropped which eventually cause the string buffers to be dropped as well.

  This probably raises a couple of questions though. How does Rust ensure that only a single variable owns
  its value?

  How can we have multiple variables point at the same data? Are we forced to copy everything to ensure only
  a single variable owns some value?

### Moves and Borrowing:

- How does rust ensure that only a single variable owns its value?

- Rust moves values to their new owner when doing things like value assignment or passing values to
  functions. ( this is important as it effects on how we write code in rust. )

  Ex: Following code:

```
    let name = "Pascal".to_string();
    let a = name;
    let b = name;
```
- Python or JavaScript: both a and b will have a reference to 'name' and therefore will both point at the
  same data.

- compling the above code rust generates error, which contain how rust expects us to handle this:

```
    error[E0382]: use of moved value: `name`
     --> src/main.rs:4:11
      |
    2 |   let name = "Pascal".to_string();
      |       ---- move occurs because `name` has type `std::string::String`, which does not implement the `Copy` trait
    3 |   let a = name;
      |           ---- value moved here
    4 |   let b = name;
      |           ^^^^ value used here after move
```

- compiler tells us that we’re trying to assign the value from 'name' to b after it had been moved to 'a'.

- The problem here is that, by the time we’re trying to assign the value of name to 'b', 'name' doesn’t
  actually own the value anymore. Why? Because ownership has been moved to 'a' in the meantime.

- Let’s take a look at what happens in memory to get a better understanding of what’s going on. When name is
  initalized, it looks very similar to our example earlier:

```
                +–––+–––+–––+
    stack frame │ • │ 8 │ 6 │ <– name
                +–│–+–––+–––+
                  │
                +–V–+–––+–––+–––+–––+–––+–––+–––+
           heap │ P │ a │ s │ c │ a │ l │   │   │
                +–––+–––+–––+–––+–––+–––+–––+–––+
```

However, when we assign the value of name to a, we move ownership to a as well, leaving name uninitialized:
```
                [–– name ––] [––– a –––]
                +–––+–––+–––+–––+–––+–––+
    stack frame │   │   │   │ • │ 8 │ 6 │ 
                +–––+–––+–––+–│–+–––+–––+
                              │
                  +–––––––––––+
                  │
                +–V–+–––+–––+–––+–––+–––+–––+–––+
           heap │ P │ a │ s │ c │ a │ l │   │   │
                +–––+–––+–––+–––+–––+–––+–––+–––+
```
- At this point, it’s no surprise that the expession let b = name will result in an error. 
- What’s important to appreciate here is that all of this is static analysis done by the compiler without
  actually running our code!

- => Rust’s compiler doesn’t allow us to write memory unsafe code.

- What if we really want to have multiple variables point at the same data, There are two ways to deal with
  this and depending on the case we want to go with one or the other. 

  Probably the easiest but also most costly way to handle this scenario is to copy or clone the value.

  Obviously, that also means we’ll end up duplicating the data in memory:

 ```
     let name = "Pascal".to_string();
     let a = name;
     let b = a.clone();
 ```
-  Notice that we don’t need to clone the value from name into a because we’re not trying to read a value
   from name after its value has been assigned to a. 

- When we run this program, the data will be represented in memory like this before its dropped:
```
            [–– name ––] [––– a –––][–––– b ––––]
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │ • │ 8 │ 6 │
            +–––+–––+–––+–│–+–––+–––+–│–+–––+–––+
                          │           │
              +–––––––––––+           +–––––––+
              │                               │
            +–V–+–––+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
```
- cloning data isn’t always an option. Depending on what data we’re dealing with, this can be a quite 
expensive operation.

- Often, all we really need is a "reference" to a value. 

This is especially useful when we write functions that don’t actually need ownership of a value. 

- Imagine a function greet() that takes a name and simply outputs it:
```
    fn greet(name: String) {
      println!("Hello, {}!", name);
    }
```
This function doesn’t need ownership to output the value it takes. 
Also, it would prevent us from calling the function multiple times with the same variable:
```
    let name = "Pascal".to_string();
    greet(name);
    greet(name); // Move happened earlier so this won't compile
```
To get a reference to a variable we use the & symbol. With that we can be explict about when we expect a
referece over a value:

```
    fn greet(name: &String) {
      println!("Hello, {}!", name);
    }
```
For the record, we would probably design this API to expect a &str instead for various reasons, but I don't
want to make it more confusing as it needs to be so we’ll just stick with a &String for now.

greet() now expects a string reference, which also enables us to call it multiple times like this:

```
    let name = "Pascal".to_string();
    greet(&name);
    greet(&name);
```
When a function expects a reference to a value, it " *borrows " it. 

Notice that it never gets ownership of the values that are being passed to it.

We can address the variable assignment from earlier in a similar fashion:

```
    let name = "Pascal".to_string();
    let a = &name;
    let b = &name;
```
With this code, name never loses ownership of its value and a and b are just pointers to the same data.
The same can be expressed with:

```
    let name = "Pascal".to_string();
    let a = &name;
    let b = a;
```
Calling greet() in between those assignments is no longer problem either:

```
    let name = "Pascal".to_string();
    let a = &name;
    greet(a);
    let b = a;
    greet(a);
```

### Return values and scope:
:
- Return values can also transfer owenership

example:
```
    fn main() {
        let s1 = gives_ownership();     // gives_ownership moves its return
                                        // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } 
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {    // gives_ownership will move its
                                        // return value into the functions
                                        // that calls it

    let some_string = String::from("yours"); // some_string comes into scope
        some_string                          // some_string is returned and
                                             // moves out to the calling
                                             // function
    }
    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { 
        // a_string comes into scope

        a_string  // a_string is returned and moves out to the calling function
    }
```
  
