# Syntax and Semantics:

1. Variable binding:

    - Bind some value to a name, so it can be used later in the program.
    - Rust convention for unused variables is they must be prefixed with "_" to avoid compiler warning.
      Rust compiler will still allocate memory for the variable on the stack, and gets included in to
      the program binary. 
    - Rust compiler does have a feature called "Dead code elimination" DEC which can remove unused code,
      including variables from binary. 
    - DCE is enabled by default in the release profile but not in the debug profile.
    - when it comes to variables that get stored on the heap, memory will still be allocated and the
      variables will be included in  the program binary. ( Rust compiler will also remove the unused
      heap allocation in the 'release' profile.)

---
    - Dead Code elimination:

        Rust's strong type system and borrow checker catches many bugs at compile time, along with that the
        compiler also prvides various attributes and directives to fine-time the build process.. 
        One such attribute is "#[allow(dead_code)]" this manages warnings related to unused code.

    dead_code warning: emited by the compiler detects code that doesn;t appear to be used or referenced
    anywhere in your program. 

    There can be a need while development to keep unused variables that will be implimented at a later
    stage. In such case when we want the compiler to silent the warning we can use #[allow(dead_code)]

    - Disabling dead_code with #[allow(dead_code)]  : instructs rust compiler to suppress the dead_code
      warning for a particular item, such as function, struct, or module. Typically placed about the item
      you wish to silence warning for.

      #[allow(dead_code)]
      fn unused_function() {}

    - when to use :
        - work in progress
        - experimental feature
        - legacy code 
---

- patterns: 
  
    - Variable binding is called a "varible", rust variable binding goes a bit further: example: 
    `let` statement is a pattern not a variable name: means we can do 
        let (x,y) = (1, 3.14)

    - patterns are used in variable binding, match expressions and other places in rust.

- type annotations:

    - Rust is statically typed language: and types come after a colon (:)

        let x: i32 = 5;

        x is binding with the type i32 and the value is 5

- Mutability:

    - By default bindings are immutable.
    - compiler catches if you forget to add "mut" 

        let mut x: u32 = 342;
        x = x / 2;

- Initializing Bindings:

    - Rust does not allow the useage of values that are not been Initialized.

    example:``
        
        let x:i32;
        println!("value of x is {x}");

- Scope and Shadowing:

    - varible bindings have scope. they are constrained to live in the block they were defined.
    - block is the collection of statmetns enclosed in between { and }. 

    ex:
        let x: i32 = 123;
        {
            let y:u32 = 2;
            println!("value of x is {x} and value of y is {y}");
        }
        println!(" the value of x is {x} and value of y is {y}"); // work 
        

