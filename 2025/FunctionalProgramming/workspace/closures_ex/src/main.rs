fn main() {
    // 1. By reference (&T) - immutable borrow
    let x = 10;
    let print_x = || println!("x = {}", x); // **borrows x immutably**
    print_x();
    println!("x is still accessible: {}", x); // x is still usable here as x is immutable
    println!("As Closure only borrows immutably and does not take ownership");

    // 2. By mutable reference (&mut T) - mutable borrow
    let mut y = 20;
    let mut increment_y = || {
        y += 1; // borrows y mutably
        println!("y inside closure: {}", y);
    };
    increment_y();
    // println!("y after closure: {}", y); // This would cause error!
    // We can't use y here because mutable borrow is still active

    // Let's demonstrate mutable reference properly
    let mut z = 30;
    {
        let mut modify_z = || z += 5;
        modify_z();
    } // mutable borrow ends here
    println!("z after mutable closure: {}", z); // Now we can use z again

    // 3. By value (T) - takes ownership
    let name = String::from("Alice");
    let consume_name = move || {
        // `move` keyword transfers ownership
        println!("Hello, {}!", name);
        // name is moved into the closure
    };
    consume_name();
    // println!("name: {}", name); // This would cause error!
    // name is no longer accessible here because ownership was transferred

    // Demonstrating move with primitive types (Copy trait)
    let age = 25;
    let take_age = move || {
        println!("Age: {}", age);
    };
    take_age();
    println!("age is still accessible: {}", age); // Works because i32 is Copy

    // Complex example showing the differences
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut vec3 = vec![7, 8, 9];

    // By reference
    let sum_vec1 = || {
        let total: i32 = vec1.iter().sum();
        println!("Sum of vec1: {}", total);
    };
    sum_vec1();
    println!("vec1 is still available: {:?}", vec1);

    // By mutable reference
    let mut clear_vec3 = || {
        vec3.clear();
        vec3.push(10);
        println!("Modified vec3: {:?}", vec3);
    };
    clear_vec3();
    // vec3 is now modified and we can use it again
    println!("vec3 after modification: {:?}", vec3);

    // By value
    let consume_vec2 = move || {
        println!("Consuming vec2: {:?}", vec2);
        // vec2 is moved here
    };
    consume_vec2();
    // println!("vec2: {:?}", vec2); // Error - vec2 was moved
    //
    demonstrate_closure_types();
}

fn demonstrate_closure_types() {
    let counter = 0;

    // Fn closure - borrows immutably
    let read_counter = || println!("Counter: {}", counter);

    let mut mutable_data = 100;

    // FnMut closure - borrows mutably
    let mut update_data = || {
        mutable_data += 1;
        println!("Updated data: {}", mutable_data);
    };

    let owned_data = String::from("test");

    // FnOnce closure - takes ownership
    let consume_data = move || {
        println!("Consuming: {}", owned_data);
        // owned_data is dropped here
    };

    read_counter();
    update_data();
    consume_data();
    // consume_data(); // This would error - closure can only be called once
}
