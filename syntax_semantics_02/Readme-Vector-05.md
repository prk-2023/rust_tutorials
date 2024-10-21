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

Source example demonstrates the following method operations:

*   Creating a vector using `Vec::new()`.
*   Adding elements to the vector using `push()`.
*   Accessing elements of the vector using indexing (`my_vector[0]`) or the `get()` method.
*   Updating the value at an index using indexing (`my_vector[0] = 100`).
*   Adding a new element to the vector using `push()`.
*   Deleting an element from the vector using `remove()`.
*   Checking the length of the vector using `len()`.
*   Checking if the vector is empty using `is_empty()`.
*   Checking the capacity of the vector using `capacity()`.
*   Iterating over the vector using a `for` loop.
*   Sorting the vector using `sort()`.
*   Reversing the vector using `reverse()`.

- Additional memory-related points for vectors:

    1.  Memory Allocation: 
        Vectors in Rust allocate memory on the heap. When you create a vector, Rust allocates a contiguous 
        block of memory to store the vector's elements.

    2. Capacity and Reallocation: 
        Vectors have a capacity, which is the amount of memory allocated to store elements. 
        When you add more elements than the current capacity, Rust reallocate the vector to a larger 
        capacity, which can be expensive. You can use `reserve()` or `resize()` to preallocate memory and 
        avoid reallocations.

    3. Ownership and Borrowing:
        Vectors in Rust follow the ownership and borrowing rules. When you create a vector, you own the 
        memory allocated to it. 
        You can borrow the vector's elements using references (`&`) or mutable references (`&mut`).

    4.  Deref and DerefMut: 
        Vectors implement the `Deref` and `DerefMut` traits, which allow you to dereference the vector and 
        access its elements directly.

    5. Drop and Deallocate:
        When a vector goes out of scope, Rust automatically deallocates the memory allocated to it using 
        the `Drop` trait. You can also use `clear()` to remove all elements and deallocate the memory.

    6.  **Clone and Copy**: 
        Vectors implement the `Clone` and `Copy` traits, which allow you to create a copy of the vector or 
        its elements. However, cloning a large vector can be expensive.

    7.  Iterators and Ownership**: 
        When you iterate over a vector using an iterator, the iterator takes ownership of the vector's 
        elements. You can use `iter()` or `into_iter()` to create an iterator that borrows or takes 
        ownership of the elements.

    8.  Vector Slicing: 
        Vectors support slicing, which allows you to create a reference to a subset of the vector's
        elements. Slicing does not allocate new memory; it only creates a reference to the existing memory.

Source example demonstrates the following points:

*   Reserving memory to avoid reallocations using `reserve()`.
*   Borrowing the vector's elements using references (`&`).
*   Derefing the vector using `Deref`.
*   Clearing the vector and deallocating memory using `clear()`.
*   Cloning the vector using `clone()`.
*   Creating an iterator that borrows the vector's elements using `iter()`.
*   Creating an iterator that takes ownership of the vector's elements using `into_iter()`.
*   Slicing the vector using indexing (`&my_vector[1..3]`).
