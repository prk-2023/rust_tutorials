use std::io;
// Derive the Debug  trait for bring clone and copy traits for clone and copy trait methods on variables
#[derive(Debug, Clone)]

struct SomeStruct {
    num: i32,
}

//function to print the structure
fn print_some_struct(the_struct: SomeStruct) {
    println!("{:?}", the_struct); // :? tells the formater to use the implementation of debug to print the                                   // struct.
}

#[allow(dead_code)]
fn main() {
    // Ownership and scope :
    //let mut my_struct: SomeStruct = SomeStruct { num: 100 };
    // or rust can inver the data type
    let mut my_struct = SomeStruct { num: 100 };
    print_some_struct(my_struct.clone());

    //try to print this struct twice we will get error as the value my_struct gets moved
    //In rust by default when we pass a variable into a fun the fun takes the ownership
    //of the memory of that variable.
    //When the function runs through the variable gets dropped for leaving the scope.
    // When we try print the struct second time we will get error as the variable has moved and is
    // dropped.
    //print_some_struct(my_Struct);

    my_struct.num *= 2;
    print_some_struct(my_struct);

    // move ownership
    let name = "hello again".to_string();
    let a = &name; // a is the reference to "name" ( no ownership transfer )
    let b = a; //  create new reference to same string ( no ownership transfer )
    let new_name = name; // Ownership is transfered
                         //println!(" name : {name}");
    println!(" name : {new_name}");
    //println!(" name : {name}");

    // Strings:
    // Rust has 2 ways to represent string: string literals ( &str ) and StringType "String"
    // literals are immutable, these are referenced as "slice"
    // Note: &str is the rust primitive type and not a part of the "std" library.
    // Stored on stack and get allocated while compilation time

    // 2. String Type: growable and muttable type and is a part of the rust std lib.
    // String type : this is a struct that contains a vector of bytes, along with some metadata
    // which holds length of the string and capacity of the vector.
    // Note: when we use the String type its not required to import std , and it can be used
    // directly as below example:
    // String is stored on stack and its contents are placed on Heap.
    //
    // ex: of string Type:
    let mut s: String = String::from("Hello ");
    s.push_str(" Rust World! ");

    // read input from keyboad:
    let mut test_string = String::new();
    let _ = io::stdin().read_line(&mut test_string);

    let s1 = String::from("hello world");
    let _s2 = take_ownership(s1);
    //println!("{}", s1);  // <-- ownership is transfered

    //Stack variables "copy" trait:
    /* In Rust, when you assign a value to a new variable, Rust checks if the type of the
     * value implements the Copy trait. If it does, Rust creates a copy of the value and
     * assigns it to the new variable. If it doesn't, Rust moves the value to the new variable,
     * transferring ownership.
     *
     * The Copy trait is automatically implemented for types that are cheap to copy, such as
     * integers, floats, and booleans. This is why you can assign x1 to x2 without transferring
     * ownership.
     *
     * If you were to use a type that does not implement the Copy trait, such as a String,
     * the assignment would transfer ownership as in the above example
     */

    let x1 = 5;
    let x2 = x1;

    /* x1 and x2 are separate copies of the same value, you can use both x1 and x2 without any issues. */

    println!(
        "This is to show all primitive types implement a copy trait and x1, x2 = {},{}",
        x1, x2
    );

    // moving ownerhship back and forth is inconvinent and rust solves with borrowing:
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
