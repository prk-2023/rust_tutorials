use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
fn main() {
    let x = 5;
    let y = Some("hello");

    print_type_of(&x);
    print_type_of(&y);
}
