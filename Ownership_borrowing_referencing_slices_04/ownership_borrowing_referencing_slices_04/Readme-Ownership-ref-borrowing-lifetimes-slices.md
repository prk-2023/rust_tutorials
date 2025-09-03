# Ownership, Reference, Borrowing, Slicing:
---

Meta: Before learning about Ownership there are 2 important aspects to note:

1. Rust aims at speed and security and it achives this with "zero-cost abstractions". 
Zero Cost Abstractions: Rust abstractions cost as little as possible in order to make them work.
Ownership is a zero-cost anstraction as all the analysis is done at Compilation time and does not show in
the run-time cost of these features.

2. sharp learning curve: the programmers mental model does not match the actual rules set by Rust which
   leads to longer learning curve.

## Ownership:

Ownership in Rust refers to the relationship between a value and the variable that owns it.
The __Owner__ of the value is responsible for **allocation and deallocating** the value when not required.

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

- These rules have deep implications for the rust language.

- Ownership gives memory safety guarantees without the need for garbage collectors.

- Borrowing, reference slices are related to ownership and are required to understand how rust operates.

- Ownership helps prevent common programming erros like null ptr dereferences, data race conditions.

- Rust Ownership deals with managing the memory and lifetime of values. 

=> Rust has a different approch for managing memory through a system of ownership with a set of rules that
the compiler checks.

  * If Any of these rules get violated then the program will not compile.

  This concept of ownership is a new compared with the current systems programming languages. And
  understanding the concept of ownership is mandatory for developing code taht is safe and efficient.

- In Rust, every value has an owner that is responsible for managing its memory. 
  When a value is created, it is is assigned to a variable, which becomes its owner. 

  The owner is responsible for ensuring that the value is valid and accessible for as long as it's needed.

## **Rules of Ownership**:  

    There are three rules of ownership in Rust:
    1. **Each value in Rust has an owner.**
    2. **There can only be one owner at a time.**
    3. **When the owner goes out of scope, the value will be dropped.**

- The rules that govern how Rust program manages memory.
- Ownership gives memory safety guarantees without the need for garbage collectors.
- Borrowing, reference slices are related to ownership and proper understanding is required for programming
  in rust.
  - Ownership helps prevent common programming erros like null ptr dereferences, data race conditions.
  - Ownership also deals with managing the memory and lifetime of values.
  - The Rust ownership rules are checked by the compiler specifically by the borrow checker.


### **Borrow Checker**: 

    Its a part of Rust compiler that checks the code at compilation time and enforces the code(variables ) 
    satisfies the ownership rules. 

    Borrow checker checks: ( static / compilation checks )

        - Ownership: verify each value has only a single owner, and owner is responsible for deallocating
          the value when its no longer needed.

        - Borrowing: make sure the borrowing is done correctly, and the borrowing rules are followed.
          For example it checks if the value is not borrowed as mutable and immutable at same time 

        - Lifetime: The borrow checker ensures that the lifetimes of values are correctly managed, and
          that values are not used after they have gone out of scope.

    Borrow checker is a key feature of the Rust language, and it helps to prevent common errors such 
    as null pointer dereferences, use-after-free bugs, and data corruption.

    Drop System ( runtime check ):
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

    - C, C++ language design overlooks the property of memory safety, and puts the responsibility on the
      programmer and the design of the application. ( this is generally error prone as there are many
      constrains which might be overlooked by the programmer or by the programs design )

- mem safety is a correctness issue: 

    A memory unsafe program may crash or produce non-deterministic output depending on the bug.

  - There are many languages that allow us to write "memory unsafe" code in the sense that it's fairly easy
    to understand bugs. 
    Ex:
    - Dangling pointer: pts that point to invalid data ( this will be more clear when we look at how data is
      stored in memory). (https://stackoverflow.com/questions/17997228/what-is-a-dangling-pointer)
    - Double free: trying to free that same memory location more then once, this can leade to
      "undeterministic behaviour".(https://stackoverflow.com/questions/21057393/what-does-double-free-mean))
    - Unlike languages that come with GC, Rust uses the concept of owenship, Borrowing to handle issues
      related to memory.

    => So when we say rust comes with memory safety, we refer to the fact that by default Rust compiler does
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

- The ptr to the heap is known and fixed size => you can store the ptr on the stack, but the actual data is
  stored on heap memory and the pointer points to the address on the heap.

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

- By default a reference is read-only :
    
    ```rust 

        fn foo (v: &Vec<i32) {
            v.push(5);
        }
        let v = vec![];
        foo(&v);
    ```
    the above code will give error :
        error: cannot borrow immutable borrowed content `*v` as mutavle v.push(5);

    -> push a value mutates the vector and we are not allows as v is the by default immutable.

- &mut reference: these are second kind of references :
    
        `&mut T` 

    Mutable reference allows you to mutate the resource you're borrowing Ex:

    ```rust 
        let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
        println!("{}", x); // this outputs 6
    ```
    Here we made `y` a mutable reference to `x` and then add one to the the thing `y` points at.

    Note In the above example `x` also need to be marked/defined as mutable variable.

    `*y`: because y is `&mut` reference, we need to use `*` to access the contents of the reference.

    - In the above example if we remote { } ( i.e remove that extra scope ) the program will error on
      compilation:

      ```
      error : cannot borrow `x` as immutable because it is also borrowed as mutable
        println!("{}", x);
                       ^
        
      note: previous borrow of `x` occurs here; the mutable borrow prevents subsequent moves, borrows, 
      or modification of `x` until the borrow ends

        let y = &mut x;
                     ^

      note: previous borrow ends here
      fn main() {

      }
      ^
      ```

      => there are Rules while using References.

- Borrowing Rules:
    
    First, any borrow must last for a scope no greater than that of the owner. 
    Second, you may have one or the other of these two kinds of borrows, but not both at the same time:

    1. one or more references (&T) to a resource,
    2. exactly one mutable reference (&mut T).
   
=>  Summing up references:

    In Rust, references are used to borrow values without taking ownership of them.
    
    There are two types of references: 
        - immutable references (`&`) 
        - mutable references (`&mut`). 

    **Mutable References**
    A mutable reference is a reference that allows you to modify the value it points to. 
    It's denoted by the `&mut` keyword. Here's an example:

    ```rust
        let mut x = 5;
        let mut_ref = &mut x;
        *mut_ref = 10;
        println!("{}", x); // prints 10
    ```

    In this example, we create a mutable reference `mut_ref` to the value `x`. 
    We can then use the dereference operator `*` to modify the value `x` through the reference.

    **Borrowing Rules**

    Rust has a set of borrowing rules that ensure memory safety.

    1. You can create multiple immutable references to the same value, and they can coexist.
    2. **You can have only one mutable reference to a value**: You can create only one mutable reference to
       a value at a time. If you try to create another mutable reference to the same value, the compiler 
       will prevent it.
    3. **You cannot have a mutable ref and an immutable ref to the same value at the same time**: If you
       have a mutable reference to a value, you cannot create an immutable reference to the same value until
       the mutable reference goes out of scope.
    4. **References must be valid for the entire duration of the borrow**: A reference must be valid for the
       entire duration of the borrow. If the reference becomes invalid (e.g., because the value it points 
       to goes out of scope), the borrow is invalid.

    Example that demonstrates the borrowing rules:

    ```rust
        let mut x = 5;
        // Create an immutable reference to x
        let imm_ref = &x;
        println!("{}", imm_ref); // prints 5
        
        // Create another immutable reference to x
        let imm_ref2 = &x;
        println!("{}", imm_ref2); // prints 5

        // Try to create a mutable reference to x
        // This will fail because we already have immutable references to x
        // let mut_ref = &mut x; // Error: cannot borrow `x` as mutable

        // Drop the immutable references
        drop(imm_ref);
        drop(imm_ref2);

        // Now we can create a mutable reference to x
        let mut_ref = &mut x;
        *mut_ref = 10;
        println!("{}", x); // prints 10

    ```

In example, we create two immutable references to `x` and then try to create a mutable reference to `x`. 
The compiler prevents us from creating the mutable reference because we already have immutable references to `x`. We then drop the immutable references and create a mutable reference to `x`, which is allowed.

**Borrow Checker**

Borrow checker checks borrowing rulest at compilation and prevents compilation if checker detects a 
borrowing error, it will prevent the code from compiling.

**Smart Pointers** [ TODO: ]

Rust's smart pointers, such as `Rc` and `Arc`, also follow the borrowing rules. 
These smart pointers provide a way to manage shared ownership of values and ensure memory safety.

- As we can only have one &mut at a time, it is impossible to have a data race. 
This is how Rust prevents data races at compile time: we’ll get errors if we break the rules.

- Use after free:
    References should not live longer then the resource they refer to. 
    Rust checks the scope you references to ensure that this is true. ( if rust does not check this
    property, we would accidentally use a reference which was invalid.

    let y: &i32;
    {
        let x = 5;
        y = &x;
    }
    println!("{}",y);

    this will give error: `x` does not live long enough.....

    or `y` is only valied for the scope where `x` exists, as `x` gets droped it becomes invalid to refer to
    it.

    Same error occues when the reference is declated before the variable it referes to. This is beacuse
    resoures within the same scope are freed in the opposite order they were declared.

    let y: &i32;
    let x = 5;
    y = &x;
    println!("{}",y);

    this will give error: `x` does not live long enough. 

    here y is declared before x meaning y lives longer then x which is not allowed.


### Lifetimes:

    Rust has a focus on safety and speed, which it accomplishes through zero-cost abstractions.
    owenership system is an example of zero-cost abstraction. 

    All the owenership analysis is done at compilation time. ( we do not have to worry of the runtime cost
    for any of the owenership features.)

- Lifetime is a concept that refers to the scope for which a reference to a value is valid.

- Lifetime is a way to specify the relationship between the lifetime of a reference and the lifetime of the
  value it references.

- Why lifetimes?
    Every reference has a lifetime, which is scope for which the reference is valid. 
    If a reference outlives the value it references, it can lead to dangling pointers, which are pointers
    that points to memory that has already been deallocated. ( this causes undefined behavioud and crashes.)

    To prevent this Rust requires you to specify the lifetime of a reference when you create it.
    This ensures that the reference is only valid for as long as the value it references exists.
    ```rust 
        let r;              // Introduce reference: `r`.
        {
            let i = 1;      // Introduce scoped value: `i`.
            r = &i;         // Store reference of `i` in `r`.
        }                   // `i` goes out of scope and is dropped.
        println!("{}", r);  // `r` still refers to `i`.
    ``` 
    error:

    to fix the above error step 4 has to be avoided after step 3. 
    The compiler check can report this at compilation time. But the issue gets more complicated when we have
    functions that take reference arguments.

    ```rust 
        fn skip_prefix(line: &str, prefix: &str) -> &str {
            // ...
          line
        }

        let line = "lang:en=Hello World!";
        let lang = "en";

        let v;
        {
            let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
            v = skip_prefix(line, p.as_str());  //  |
        }                                       // -+ `p` goes out of scope.
        println!("{}", v);
    ```
    Here we have a fun skip_prefix which takes two &str references as parameters and returns a single &str 
    reference.
    We call it by passing in references to line and p: Two variables with different lifetimes. 
    Now the safety of the println!-line depends on whether the reference returned by skip_prefix function
    references the still living line or the already dropped p string.
    Because of the above ambiguity, Rust will refuse to compile the example code.

    To get it to compile we need to tell the compiler more about the lifetimes of the references.
    This can be done by making the lifetimes explicit in the function declaration:

    ```rust 
        fn skip_prefix<'a,'b>( line: &'a str, prefix: &'b str) -> &'a { 
            // ..... 
        }
    ```
    The first change was adding the <'a, 'b> after the method name. 
        This introduces two lifetime parameters: 'a and 'b. 

    Next, each reference in the function signature was associated with one of the lifetime parameters by 
    adding the lifetime name after the &. 
    This tells the compiler how the lifetimes between different references are related.

    As a result the compiler is now able to deduce that the return value of `skip_prefix` has the same 
    lifetime as the `line` parameter, which makes the `v` reference safe to use even after the p goes out 
    of scope in the original example.

    In addition to the compiler being able to validate the usage of `skip_prefix` return value, it can also 
    ensure that the implementation follows the contract established by the function declaration. 
    This is useful especially when you are implementing traits.

- Lifetime Syntax:

    lifetimes are denoted by a single quote followed by a name, such as 'a or 'b.

    ex:
        fn foo<'a'> ( x: &'a i32) -> &'a i32 {
            x
        }

    'a : specified as parameter to "foo" function. 
    x parameter is reference to an i32 with lifetime 'a, and retuen value is also a reference to an i32 with
    a lifetime 'a.
    
    A function can have ‘generic parameters’ between the < >s of which lifetimes are one kind.
 
    We use < > to declare our lifetimes. This says that `foo` has one lifetime, 'a. 
    If we had two reference parameters with different lifetimes, it would look like this: foo<'a,'b> (...)

    In our parameter list, ...(x: &'a i32) 
    if we want &mut ref then it would be ...(x: &'a mut i32)

- In Structs: You’ll also need explicit lifetimes when working with structs that contain references:
    
    ```rust 
        struct Foo<'a> {
            x: &'a i32,
        }

        fn main() {
            let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
            let f = Foo { x: y };

            println!("{}", f.x);
        }
    ```
    Above we see Struct also have litetime. similar to functions.
    
    struct Foo< 'a >{ ...} 

    declares a lifetime and 

        x: &'a i32,

    uses it.

    The reason we need lifetime is to ensure that any reference to a `Foo` cannot outlive the reference to
    an i32 it contains.

- impl blocks:

    let's implement a method on `Foo`:

    ```rust 
        struct Foo<'a> {
            x: &'a i32,
        }

        impl<'a> Foo<'a> {
            fn x(&self) -> &'a i32 { self.x }
        }

        fn main() {
            let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
            let f = Foo { x: y };

            println!("x is: {}", f.x());
        }
    ```
    we need to declare a lifetime for `Foo` in the `impl` line.
    We repeat 'a twice, like on functions: `impl<'a>` defines a lifetime 'a, and `Foo<'a>` uses it.

- multiple lifetimes:

    If you have multiple references, you can use the same lifetime multiple times:

    #![allow(unused_variables)]
    fn main() {
        fn x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str {
            x 
        }
    }

    this says `x and y` are both alive for the same scope , and return is also alive for that scope.

    For function which have different lifetimes for `x and y`:

    #![allow(unused_variables)]
    fn main() {
        fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
           x 
        }
    }
    x and y have different valid scopes, but the return value has the same lifetime as x.

- Thinking in Scope:

    A way to think about lifetimes is to visualize the scope that a reference is valid for. 
    Ex:
    ```rust 
        fn main() {
            let y = &5;     // -+ `y` comes into scope.
                            //  |
                            // Stuff...     //  |
                            //  |
        }                   // -+ `y` goes out of scope.

    ```
    Adding in our Foo:

    ```rust 
        struct Foo<'a> {
            x: &'a i32,
        }

        fn main() {
            let y = &5;           // -+ `y` comes into scope.
            let f = Foo { x: y }; // -+ `f` comes into scope.
                                  //  |
                                  // Stuff...           
                                  //  |
                                  //  |
        }                         // -+ `f` and `y` go out of scope.
    ```
    f lives within the scope of y, so everything works.

    But wrong usage the code will fail: ex:
    ```rust 
        struct Foo<'a> {
            x: &'a i32,
        }
        
        fn main() {
            let x;                    // -+ `x` comes into scope.
                                      //  |
            {                         //  |
                let y = &5;           // ---+ `y` comes into scope.
                let f = Foo { x: y }; // ---+ `f` comes into scope.
                x = &f.x;             //  | | This causes an error.
            }                         // ---+ `f` and y go out of scope.
                                      //  |
            println!("{}", x);        //  |
        }                             // -+ `x` goes out of scope.
    ```
    The scopes of `f` and `y` are smaller than the scope of `x`. 
    But when we do x = &f.x, we make x a reference to something that’s about to go out of scope.

    Named lifetimes are a way of giving these scopes a name. 
    Giving something a name is the first step towards being able to talk about it.
   

- 'static: 

    The lifetime named ‘static’ is a special lifetime. 
    It signals that something has the lifetime of the entire program. 
    Most Rust programmers first come across 'static when dealing with strings:
    
    ```rust 
        #![allow(unused_variables)]
        fn main() {
            let x: &'static str = "Hello, world.";
        }
    ```
    String literals have the type " &'static str  " because the reference is always alive: they are baked 
    into the data segment of the final binary.

    ```rust 
    #![allow(unused_variables)]
    fn main() {
        static FOO: i32 = 5;
        let x: &'static i32 = &FOO;
    }
    ```

    Another example are globals:

    ```rust 
        #![allow(unused_variables)]
        fn main() {
            static FOO: i32 = 5;
            let x: &'static i32 = &FOO;
        }
    ``` 
    This adds an i32 to the data segment of the binary, and x is a reference to it.

- Only things relating to references (such as a struct which contains a reference) need lifetimes.

- Return values and scope:
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

#### Summing up lifetimes:

    - Lifetimes are a way to specify the scope of a reference in Rust.

    - Ensures that the reference to a value is valid for a certain period of time and the value is not
      dropped or moved while the reference is still in use.

    - Basic Lifetime Syntax:
        fn print_string(s: &'a str) {
            println!("{}", s);
        }
        Function takes a reference to a string with a lifetime of 'a. => string must be valid for atleast
        the duration of function call.

    - Lifetime parameters: USed to specify lifetime of a reference. can be used with:
        - function signature,
        - struct definitions,
        - trait definitions.

        fn print_string<'a>(s: &'a str) {
            println!("{}", s);
        }
        Fun takes a reference to a string with a lifetime of 'a. 
        'a is a lifetime parameter that is the function signature.

    - Lifetime Elision: feature that allow  you to omit the parameters in certain situations:
    Ex: if a fun takes a single reference as an argument, the lifetime parameter can be omited:
        fn print_string(s: &str) {
            println!("{}", s);
        }   
    fun takes a ref to a string , but the lifetime parameter is omitted. this is beacause the lifetime of
    the reference is inferred by the compiler.

    - Multiple lifetime parameters: Its possible to have multiple parameters in a single function signature:

        fn print_string<'a, 'b> (s1: &'a str, s2: &b' str) {
            println!("{} {}", s1, s2);
        }
       Function takes 2 references to string with different lifetimes.
       The 'b, 'a bound specify that the lifetime of s2 must be at least as long as the lifetime of s1.

    - Struct lifetime: 
        struct Person <'a >{
            name: &'a str,
            age: u8,
        }
      Person struct has a lifetiem parameter 'a that specifies the lifetime of the "name" field.

    - Trait Lifetimes:

        trait Printable <'a> {
            fn print(&self, s: &'a str);
        }
        Printable has lifetime parameter 'a that specifies the lifetime of "s" parameter.

#### Usecase of lifetimes:

    1. String manipulation: When working with strings, lifetimes are used to ensure that the string is valid
       for the duration of the operation.
    2. File I/O : When working with files, lifetimes are used to ensure that the file is open for the
       duration of the operation.
    3. Networking: When working with network connections, lifetimes are used to ensure that the connection
       is open for the duration of the operation.

#### Summery: Lifetime

    Rust has several rules that govern how lifetimes work:

    1. Lifetime parameters:  are specified using the syntax 'a, 'b, etc. 
    These parameters can be used to specify the lifetime of a reference.
    
    2. Lifetime bounds: are specified using the syntax T: 'a, where T is a type and 'a is a lifetime. 
    This means that the type T must outlive the lifetime 'a.

    3. Lifetime inference: Rust can often infer the lifetime of a reference based on the context in 
    which it is used.

    4. Lifetime subtyping: Rust allows lifetime subtyping, which means that a reference with a shorter 
    lifetime can be used where a reference with a longer lifetime is expected.
    
    Common lifetime patterns:
    
    1. Static lifetime: The static lifetime is denoted by the 'static keyword. 
    It means that a reference is valid for the entire duration of the program.

    2. Input lifetime: The input lifetime is denoted by the 'a keyword. 
    It means that a reference is valid for as long as the input value exists.

    3. Output lifetime: The output lifetime is denoted by the 'b keyword. 
    It means that a reference is valid for as long as the output value exists.

    // Example 1: Static lifetime
    let s: &'static str = "hello";
    
    // Example 2: Input lifetime
    fn foo<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    // Example 3: Output lifetime
    fn bar<'a, 'b>(x: &'a i32) -> &'b i32 {
        x
    }

    // Example 4: Lifetime bounds
    fn baz<'a, T: 'a>(x: T) -> &'a T {
        &x
    }
    
 -----------------------------------
 #### copy and clone:

    `Copy` and `Clone` are two traits that allow to create a copy of a value, but they serve different
    purpose and have different implications.

    * **Copy**:

    The `Copy` trait is used to create a copy of a value by simply copying its bits. 
    This is a shallow copy, meaning that it only copies the top-level value and does not recursively 
    copy any nested values.

    When you implement the `Copy` trait for a type, you're indicating that the type can be safely copied 
    by simply copying its bits. This is typically used for small, primitive types like integers, booleans, 
    and characters.
    While implmenting `copy trait` its important to keep in mind the complexity of copy and how expensive
    the implementation can be.

    example:
    ```rust
        let x = 5; // x is a Copy type
        let y = x; // y is a copy of x
    ```
    In this example, `y` is a copy of `x`, and both `x` and `y` can be used independently.

    * **Clone**

    `Clone` trait is used to create a deep copy of a value. 
    This means that it recursively copies all nested values, not just the top-level value.

    When you implement the `Clone` trait for a type, you're indicating that the type can be safely cloned by
    recursively copying all its nested values. 
    This is typically used for more complex types like structs, enums, and collections.

    example:
    ```rust
        #[derive(Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let person = Person {
            name: "John".to_string(),
            age: 30,
        };

        let cloned_person = person.clone(); // cloned_person is a deep copy of person
    ```
    In the example, `cloned_person` is a deep copy of `person`, and both `person` and `cloned_person` can be used independently.


    - **Key differences**

    * **Shallow vs. deep copy**: `Copy` creates a shallow copy, while `Clone` creates a deep copy.
    * **Performance**: `Copy` is typically faster than `Clone`, since it only copies the top-level value.
    * **Safety**: `Copy` is safer than `Clone`, since it doesn't recursively copy nested values.
    * **Usage**: `Copy` is used for small, primitive types, while `Clone` is used for more complex types.

    **When to use each**

    * Use `Copy` when:
        + Working with small, primitive types.
        + Need a shallow copy of a value.
        + want to ensure safety and performance.

    * Use `Clone` when:
        + Working with complex types like structs, enums, or collections.
        + Need a deep copy of a value.
        + Willing to pay the performance cost of recursively copying nested values.
-------------------------------
#### Similar Concepts in Other Programming Languages as lifetime:

Rust's concept of lifetimes is unique, other programming languages have similar concepts that aim to ensure
memory safety and prevent common errors like use-after-free and data corruption. 

Few examples:

1. CPP : Smart Pointers: 
    CPP 11  introduced smart pointers like `std::unique_ptr` and `std::shared_ptr`, which provide automatic
    memory management and prevent use-after-free errors.

2. CPP: RAII (Resource Acquisition Is Initialization): 
    RAII is a technique that binds the life cycle of a resource (like a file or a lock) to the life cycle 
    of an object. This ensures that resources are properly released when they are no longer needed.

3. Java: Garbage Collection: 
    Java's garbage collector automatically manages memory and prevents use-after-free errors. 
    However, Java's garbage collector does not provide the same level of control as Rust's lifetimes.

4. C#: Dispose Pattern: 
    C#'s Dispose pattern is similar to RAII in C++. It provides a way to release resources when they are 
    no longer needed, but it does not provide the same level of control as Rust's lifetimes.

5. Swift: ARC (Automatic Reference Counting): 
    Swift's ARC is similar to C++'s smart pointers. It provides automatic memory management and prevents 
    use-after-free errors.

6. Haskell: Linear Types: Haskell's linear types are similar to Rust's lifetimes. 
    They ensure that resources are used exactly once and prevent use-after-free errors.

7. Kotlin: Coroutines: Kotlin's coroutines provide a way to manage the life cycle of asynchronous 
   operations and prevent use-after-free errors.

- Comparison with Rust Lifetimes:

While these concepts share similarities with Rust lifetimes, they differ in several ways:

* **Explicitness**: Rust lifetimes are explicit and require the programmer to specify the lifetime of each
  reference. In contrast, many other languages use implicit lifetimes or garbage collection.

* **Control**: Rust lifetimes provide fine-grained control over the life cycle of references, allowing
  programmers to specify the exact lifetime of each reference. Other languages often provide less control 
  over the life cycle of resources.

* **Memory Safety**: Rust lifetimes are designed to prevent use-after-free errors and ensure memory safety.
  While other languages may provide some level of memory safety, they often rely on garbage collection or
  other mechanisms that can be less effective.

### Slice: ( Type of Reference )

- Slice is a type of reference that refers to a continuous sequence of elements in a collection, such as
  arrays or vectors.

- Slices are similar to references, but they can refer to a sub-system of elements in a collection, rather
  then the entire collection.

- Create a Slice: A slice can be created using "&" operator followed by the range of elements you want to
  include in the slice.
    
  Example:
    
        let arr = [1,2,3,4,5];
        let slice = &arr[1..3]; // this creates a Slice of element 2 and 3 
        println!("{:?}", arr);  // prints 2,3
        println!("{}", arr[0]);  // prints 2
        println!("{}", arr[1]);  // prints 3

  => slice is a reference to the element 2 , 3 in the arr array.

    Syntax:

    &arr[start..end];

    - start: is the index of the first element in the slice.
    - end: is the index of the last element in the slice.
    - If 'start' is omitted then it defaults to 0th index.
    - If 'end' is omitted then it defaults to the length of the array.

  Ex:
        let arr = [1, 2, 3, 4, 5];
        let slice1 = &arr[..];   // <-- creates a slice of all the elements.
        let slice1 = &arr[1..];   // <-- creates a slice of 2,3,4,5
        let slice1 = &arr[..3];   // <-- creates a slice of 1,2,3

- In Rust we have 2 typs of slieces.

    1. Shared Slice:
        Is the reference to a slice that can be shared with multiple parts of the code.
        Its created using & opeartor.
    2. Mutable Slice:
        Its a reference to a slice that can be modified. 
        Its created using "&mut" oprator.

    ex:

        let arr = [1, 2, 3, 4, 5];
        let shared_slice = &arr[..];    //creates a shared slice
        let mut_slice = &mut arr[..];    //creates a mutable slice

#### Slice Methods:

    Slices have several methods that can be used to manipulate them. 
    Some common methods inclide:

    - len(): returns the length of the slice 
    - iter(): returns an iterator over the elments of the slice.
    - get(): returns a reference to the element at the specified index.
    - get_mut(): returns a mutable reference to the element at the specified index.

    Ex:
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[..];
        println!("{}", slice.len()); // prints 5
        for elem in slice.iter() { 
            println!("{}", elem);
        }

#### Slice safety:

    1. Slice can be source of errors if not used carefully. 
    2. Slice safety refer to the property of a slice that ensures it does not outlive the data it
       references. 
       i.e Slice is safe if it does not attempt to access memory that has already been deallocated or
       reused.

    3. Common mis-usages of slices:
        - Use-after-free: Access memory that has already been deallocated.
        - Data Corruption: modify data that no longer exists.
        - Dangling Pointers: pointing to memory that is no longer valid.

#### Slice recommended usage:
    
    - Use "lifetime" annotations: use the & operator to create instead of using raw-pointers.
    - Use & operator: Use the & opertor to create slices instead of using raw pointers.
    - Avoid returning slices from functions: Avoid returing slices from functions unless you are sure that
      the slice will outlive the date it references.
    - Avoid storing slices in structs: Avoid storing slices in structs unless you are sure that the slice
      will not outlive the data it references.


