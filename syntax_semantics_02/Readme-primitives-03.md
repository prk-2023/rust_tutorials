# primitives types:

- Rust primitives are built-in to the language. 
- Rust Standard librarau also provides a number of useful types built on top of the primitives. 

- Most primitive inbuilt :
    Boolean     : true, false
    char        : represents a single unicode scalar value, We can create a char S with a single ' 
                  let x = 'x';
                  Rust char is 4 bytes. 
    Numerical   : Rust has varity of numetic types: signedm unsigned, fixed and variable, floating point &
                  integer.
 
            i8, i16, i32, i64: Integer
            u8, u16, u32, u64: Unsigned Integer
            isize, usize : arch dependent variable size ( Their range is sufficient to express the size 
                                of any collection, so these types have ‘size’ as the category. )
            f32 : siggle precision float  
            f64 : double precision float

    Array       : fixed-size list of elements of the same type. By default, arrays are immutable.

        let a = [1, 2, 3]; // a: [i32; 3]
        let mut m = [1, 2, 3]; // m: [i32; 3]

        Arrays have a type  [T; N]  ( T the generic notation , N is the compile-time constant for length of
        the array.)

    shorthand for initializing each element of an array to the same value.

        let a = [0;20]; // a: [i32; 20]

    get the number of elements in a array:
        a.len() 

    Slices: Slice is another data structure. Theya re usefil for allowing safe efficient access to a portion
    of an array without copying. 
    ex: reference only one line of a file that is read into memory. 
    By definition slice is not created directly, but from an existing variable binding.
    Slices have a defined length, and can be mutable or immutable.

    Internally, slices are represented as a pointer to the beginning of the data and a length.

    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in `a`.
    let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.

    slice are not owners, they created by borrowing a portion of the underlying data and they do not own the
    data themselves.

    useful when we want to work with a large collection without taking ownership of the entire collection.  
    useful to share data between multiple parts of the program without creating multiple copies of the data.


    Some key characteristics of slices in Rust are:
    1. slices are references, not values. They do not own the data they reference.
    2. slices are created by borrowing a portion of the underlying data.
    3. Slices cannot outlive the data they reference.
    4. Slices are immutable by default, but you can create mutable slices by using the &mut keyword.


    There are several types of slices in Rust, including:
    1. &[T]: a shared slice of type T
    2. &mut [T]: a mutable slice of type T
    3. &str: a shared slice of characters (a string)
    4. &mut str: a mutable slice of characters (a string)

    - str: most primitive string type. 
    Not very useful by itself, but becomes useful when placed behind a reference, like &str (string slice)

    Tuple: A tuple is an ordered list of fixed size. Like this:
    
        let x = (1, "hello");

        or 

        let x : (i32, &str) = (1, "hello");
        

    Access the fields in a tuple through a destructuring let. Here’s an example:
        
        let (x, y, z) = (1, 2, 3);
        println!("x is {}", x);


    (0,); // A single-element tuple.
    (0); // A zero in parentheses.
    
    Access Tuple with indexing: indexing starts at zero, but unlike array indexing, it uses a .,
                                rather than []s.

        let tuple = (1, 2, 3);
        let x = tuple.0;
        let y = tuple.1;
        let z = tuple.2;
        println!("x is {}", x);
 
    - Function type: functions also have type! 

        #![allow(unused_variables)]
        fn main() {
            fn foo(x: i32) -> i32 { x }
            let x: fn(i32) -> i32 = foo;
        }

    x is a ‘function pointer’ to a function that takes an i32 and returns an i32.
