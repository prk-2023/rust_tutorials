use std::collections::HashMap;
// Vectors: Collections:
// Collections are associated with dynamic and stored on the heap, so they can shrink at runtime.
// Vectors are use used commonly used collection.
// Vectors : Growable arrays, Push, Pop, Index, Iterate ...
fn print_vector(v: &Vec<i32>) {
    println!("---");
    for i in v {
        println!("{:?}", i);
    }
}
fn main() {
    {
        // Vectors
        // Create an empty Vector and add elements later
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        // for i in &v { // make sure to pass the address :borrow checker checks for moving
        //     println!("{}", &i);
        // }
        print_vector(&v);
        v.pop(); //removes last element
        print_vector(&v);

        // using macros ( using vec! macro  for auto code generation at compile time )
        let mut v = vec![10, 11, 12, 13];
        v.push(14);
        print_vector(&v);
        v.pop(); //removes last element
        print_vector(&v);

        //other vector operations
        // Read elements via
        // 1. using get method
        // 2. using indexing

        // Access values of Vectors:
        // ----------------------
        // 1. using get()
        assert_eq!(Some(&10), v.get(0));
        match v.get(0) {
            Some(val) => println!("Element at the first index(0) : {}", val),
            None => println!("No value at the index"),
        }
        // or use .map()
        // map is a method on Option<T>
        // next apply a closure (|x| println!("{}",x)) only if the Option is Some(x)
        // if Option is None nothing happens i.e the closure does not run.

        //  should print 10
        v.get(0).map(|x| println!("{}", x));
        // Nothing will be printed even we touch a index that does not exit
        v.get(100000)
            .map(|x| println!("nothing should be printed at this index {}", x));

        v.push(6);
        let maybe_fourth = v.get(3); // returns Option<&i32>
        print_vector(&v);

        let mut x = vec![1, 2, 3, 4, 5];
        // Immutable borrow occurs below
        let first = &x[0];
        //x.push(6); //error: cannot borrow `x` as mutable because it is also borrowed immutably
        println!("First is: {}", first); // Immutable borrow is still in use here
        x.push(6); // previous borrow is no longer active.

        // mutate elements in a mutable vector
        for val in &mut x {
            *val += 50; // need to dereference *val to modify the actual element
        }
        print_vector(&x);

        //2. using indexing:
        let third = &x[2]; // gets a reference to the element at index 2

        // borrow is active and we can not push elements to x.
        // x.push(100); // Unable to push as borrow is active
        // After this x is borrowed and borrow mutable and borrow is active
        // we can not update x till borrows becomes de-active
        println!("Access Value at index 2 using indexing {}", third);
        x.push(100); // Unable to push as borrow is active

        // -------------------------------------------------------------------
        // Vectors : require all elements of vector to hold same type.
        // But using enum as vector elements we can make a vector hold different types of elements
        //
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.8),
        ];
        // let idx0 = &row[0]; // take the reference
        for idx in &row {
            match idx {
                SpreadSheetCell::Int(i) => println!("element is int {}", i),
                SpreadSheetCell::Text(i) => println!("element is String {}", i),
                SpreadSheetCell::Float(i) => println!("element is float {}", i),
            }
        }
    }
    {
        //string : Vec<u8> and guaranteed to form a utf-8
        //with string you can mut it extend it and combine it .
        // Create a mutable srring from literal ( there are two ways as below )
        let mut s1 = String::new(); // creates a mutable empty string
        let mut s1 = String::from("hello");
        let mut s1 = "hello".to_string();
        //Update a string: using push_str(&str): append string slice
        s1.push_str(" World!");
        // append a single character using push(char)
        s1.push('!');

        // Concatenation: can be done using format!() or "+"
        // using "+" operator: Below s1 is consumed and can not be used after Concatenation.
        // that is ownership is consumed
        let mut s1 = String::from("Hello, ");
        let mut s2 = String::from("world!");
        // s1 is moved; s2 is borrowed
        let mut s3 = s1 + &s2;
        //println!("{}", s1); // error s1 is moved

        //Concatenate with out taking ownership:
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = format!("{s1}{s2}");
        println!("Using format does not take ownership {}", s1); // No error s1 is moved
        println!("{}", s3);

        // Unlike Vectors Strings can not be directly be indexed with `[i]`.
        // since rust str are utf-8 and a single unicode char may occupy multiple bytes.
        // this prevents splitting of encoding to produce invalid data.
        // Instead we can slice string by range; but you must make sure slice boundaries align with
        // valid character (UTF-8) boundaries.
        let hello = "Здравствуйте";
        let s = &hello[0..4]; // gets a &str containing the first two Cyrillic chars
                              // If slice boundaries are invalid code will panic at runtime.
        println!("slice : {}", s);

        // Iteration
        // iterate over strong safely: in terms of characters or bytes:
        // `.chars()`: iterate over `char` ( unicode scalar)
        // `.bytes()`: iterate over raw bytes
        for c in hello.chars() {
            print!("{} : ", c);
        }
        print!("\n");
    }
    {
        //Hash Maps: provide a way to key: value ( similar to dictionaries or maps in other langs )
        // this requires an import  ( hash map is  struct type )
        //use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Blue"), 10);

        let team = String::from("Blue");
        let score = scores.get(&team); // Option<&i32>
        let x = score.map(|x| (x));
        println!(
            "reading hashmap value using method get() which gives Option<T> : {:?}",
            x
        );
        // or
        match x {
            Some(val) => println!("value read: {}", val),
            None => println!("No value "),
        }

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
        /* Word Counting*/
        {
            let text = "hello world wonderful world hello world world world!";
            let mut map = HashMap::new();

            //split on white space, count each word’s occurrences,
            //and insert or increment accordingly.
            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }
            println!("{:?}", map);
        }
    }
}
