# Vectors:

- Vector is a dynamic or 'growable' array, implemented as a std library type Vec<T> .
    T means that we can have vecotors of any type. 

- Vectors always allocate memory on heap.

- vec! macro is used to create a vector.

    let v = vec![1,2,3,4,5]; // v: Vec<i32>

- Alternative form of vec! for repeating an initial value:

    let v = vec![0; 10]; // Vector of ten zeros.

- Vectors store their contents as contiguous arrays of T on the heap. This requires to be able to know the
  size T at compilation time. 
  Size of some things can't be known at compile time. For these we have to store a pointer to that thing:
  the "Box" type works perfectly for this.

- Accessing elements:

    let v = vec![1, 2, 3, 4, 5];
    println!("The third element of v is {}", v[2]);

- out of bound access:

    let v = vec![1, 2, 3];
    println!("Item 7 is {}", v[7]);

    This will cause the current thread will panic with a message like this:

    " thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 7' "

- If you want to handle out-of-bounds errors without panicking, you can use methods like "get" or "get_mut"
  that return None when given an invalid index:

    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

- Iterating:

    we can iterate with "for":

    #![allow(unused_variables)]
    fn main() {
        let mut v = vec![1, 2, 3, 4, 5];
        for i in &v {
            println!("A reference to {}", i);
        }

        for i in &mut v {
            println!("A mutable reference to {}", i);
        }

        for i in v {
            println!("Take ownership of the vector and its element {}", i);
        }
    }

- Note: 
    - You cannot use the vector again once you have iterated by taking ownership of the vector. 
    - You can iterate the vector multiple times by taking a reference to the vector whilst iterating. 

Ex: the below example will not compile 

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    the below example can compile 

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("This is a reference to {}", i);
    }
    for i in &v {
        println!("This is a reference to {}", i);
    }
    

- Vectors have many methods ( refer to the API doc's)



