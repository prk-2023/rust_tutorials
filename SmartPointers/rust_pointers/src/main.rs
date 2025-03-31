#![allow(unused_variables)]
fn main() {
    // immutable references
    let x = 5;
    let y = &x; // y is an immutable ptr to x
    let z = &x; // z is an immutable ptr to x
    println!("{}", y);
    println!("{}", z); // Multiple references of immutable variables is allowed.

    // Mutable references:
    let mut x = 5;
    let y = &mut x; // y is Mutable references to x
    *y += 1; // modify x through y
    println!("{}", x); // prints 6
}
