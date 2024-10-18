# functions:

- Every rust program has at least one function, the "main" function.

    fn main () {

    }
- 'fn' is used to define the function followed with parentheses, which can contain arguments. arguments

    fn prime_numbers(x: i32) {
        println!("x is : {}", x);
    }
- functions that retuen type:

    fn add_one(x: i32) -> i32 {
        x + 1
    }
- functions can also return patterns 

    fn lorentz_transform_2d(x: i32, y: i32) -> (i32, i32) {
        ....
        (x + gama , y + gama )
    }

- The return value with out ";" 
  Note: Rust is expression-based language, and semicolons are different from semicolons in other "curly
  braces" and "semicolons" based languages  

- Expressions Vs Statements: 

    - Rust is primarily an expression-based language. There are only 2 kinds of statements and everthig is a
      expression.

    - expressions: return value 
    - statements: do not return values.

    This implies not all control paths return a value:

    x+1; --> this iwll not return a value.

    - Rust has 2 kinds of "declaration statements" and "expression statements". Everything else is a
      expression.

    - Declerative Statements: 
        Some lang's can use variable binding as expressions, not statements:
        ex Ruby:  x = y = y;

        In Rust we use "let" to introduce a binding is not a expression. 
        so let x = (let y = 5);  // compilation error (expected identifier, found keyword 'let'). 

        The compiler is telling us here that it was expecting to see the beginning of an expression, and a 
        'let' can only begin a statement, not an expression.

        Unlike other languages where an assignment evaluates to the assigned value (e.g. 5 in the previous 
        example), in Rust the value of an assignment is an empty tuple () because the assigned value can 
        have only one owner, and any other returned value would be too surprising:

        #![allow(unused_variables)]
        fn main() {
            let mut y = 5;
            let x = (y = 6);  // `x` has the value `()`, not `6`.
        }
        The second kind of statement in Rust is the expression statement. 
        Its purpose is to turn any expression into a statement. 
        In practical terms, Rust's grammar expects statements to follow other statements. 
        This means that you use semicolons to separate expressions from each other. 
        This means that Rust looks a lot like most other languages that require you to use semicolons at 
        the end of every line, and you will see semicolons at the end of almost every line of Rust code 
        you see.

        --> 
            fn add_one(x: i32) -> i32 {
                x + 1
            }

        the above function claims to return i32, but with a ";" it would return ( ) instead. 
        This is the reason in rust a return with ; will lead to an error.

    - summing up: 
        -  In Rust, the value of an assignment is indeed () (which is a unit type), an empty tuple. This is
           because the assigned value can only have one owner.

        - In Rust, if you omit the semicolon (;) at the end of a statement, it means that the statement is
          an expression, and its value is returned.
 
            fn foo() -> i32 {
                42 // returns 42
            }
        Here 42 is a expression and its value is returned by the function.

        - If we add a ";"  like 42; then it becomes a statement, and its value is discarded.

        - what happens when you assign a value to a variable at the end of a function?

        since the assignment is an expression that returns (), if you omit the semicolon, the () will be 
        returned by the function:

        ex:
            fn foo() -> () { 
                let x = 42 // returns ()
            }

        if you add a semicolon, the assignment becomes a statement, and its value is discarded:

            fn foo() -> () { 
                let x = 42; // does not return anything
            }
        does not retuen any thing as `let x = 42;` is a statement. and its value is discarded.

        => assignment is an expression that returns ().
        => If we remove ";" at the end it means that the statement is an expression and its value is
        returned.
        => if we add a ";", it becomes a statement and its value is discarded.

- Statements do not return values. They are executed for their side effects, such as assigning a value to a
  variable, printing to the console, or modifying a data structure.
    - let x = 42; (assigns a value to a variable, but does not return anything)
    - println!("Hello, world!"); (prints to the console, but does not return anything)
    - if true { println!("True"); } (executes a conditional statement, but does not return anything)

- Expressions, on the other hand, always return a value. This value can be used in other expressions,
  assigned to a variable, or returned by a function.
    - 42 (returns the value 42)
    - x + 42 (returns the result of the addition)
    - if true { 42 } else { 0 } (returns the value 42 or 0, depending on the condition)
        
- C and C++, blur the line between statements and expressions. In these languages, some statements can
  return values, and some expressions can have side effects. 
  Rust, on the other hand, makes a clear distinction between statements and expressions, which can make the
  language more predictable and easier to reason about.

- Diverging Functions: 
    Rust has special syntax for diverfing functions ( functions that do not return )

    fn diverges() -> ! {
        panic!("The function never returns!");
    }

    panic!()  macro similar to println!() : panic!() causes current thread of execution to crash with the
    given message. because this function will cause a crash, it will never return so it has return type "!"
    This reads a return "diverges"

    - If we add a main fun that calls deiverges() and run, it will get some output that looks like

    '' therad main panicked at the " this function never returns!", hello.rs:2 ...''

    To prit more we can get a backtrace by setting RUST_BACKTRACE env variable as below

    ```
    $ RUST_BACKTRACE=1 ./diverges 
    thread 'main' panicked at 'This function never returns!', hello.rs:2 
    Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace. 
    stack backtrace: 
        hello::deiverges 
            at ./hello.rs:2 
    hello::main 
            at ./hello.rs:6
    ```
    For full backtrace:

    ```
    $ RUST_BACKTRACE=full ./diverges 
    thread 'main' panicked at 'This function never returns!', hello.rs:2 
    stack backtrace:
    1:     0x7f402773a829 - sys::backtrace::write::h0942de78b6c02817K8r 
    2:     0x7f402773d7fc - panicking::on_panic::h3f23f9d0b5f4c91bu9w 
    3:     0x7f402773960e - rt::unwind::begin_unwind_inner::h2844b8c5e81e79558Bw 
    4:     0x7f4027738893 - rt::unwind::begin_unwind::h4375279447423903650 
    5:     0x7f4027738809 - diverges::h2266b4c4b850236beaa 
    6:     0x7f40277389e5 - main::h19bb1149c2f00ecfBaa 
    7:     0x7f402773f514 - rt::unwind::try::try_fn::h13186883479104382231 
    8:     0x7f402773d1d8 - __rust_try 
    9:     0x7f402773f201 - rt::lang_start::ha172a3ce74bb453aK5w 
    10:     0x7f4027738a19 - main 
    11:     0x7f402694ab44 - __libc_start_main 
    12:     0x7f40277386c8 - <unknown> 
    13:                0x0 - <unknown>
    ```
- RUST_BACKTRACE also works with Cargo’s run command:

- A diverging function can be used as any type: 

    ```
    #![allow(unused_variables)]
    fn main() {
    fn diverges() -> ! {
       panic!("This function never returns!");
    }
    
    let x: i32 = diverges();
    let x: String = diverges();
    }
    ```


- Function Pointers:

    - We can also create variable bindings which point to functions:

    Ex: 
        #![allow(unused_variables)]
        fn main() {
            let f: fn(i32) -> i32;
        }

        f is variable binding which points to a function that takes an i32 as an argument and returns i32.

    example:

        ```
        #![allow(unused_variables)] 
        fn main() {
            fn plus_one(i: i32) -> i32 {
                i + 1 
            }

        // Without type inference:
        let f: fn(i32) -> i32 = plus_one; 
        
        // With type inference:
        let f = plus_one;
        }
        ``` 
        We can then use f to call the function:

        ``` 
            #![allow(unused_variables)]
            fn main() {
                fn plus_one(i: i32) -> i32 { i + 1 }
                let f = plus_one;
                let six = f(5);
            }`
        ```
