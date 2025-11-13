use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// Implement a custom Smart Pointer
// 1. Define a struct that
// 2. implement methods `Deref` `DerefMut` and `Drop`
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox, cleaning up!");
    }
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    // 1.0 Custom Smart Pointer MyBox similar to Box
    let m = MyBox::new(String::from("Rustacean"));
    greet(&m); // Deref coercion: &MyBox<String> → &String → &str

    // 2.0 Rc<T> ( Reference Count Smart pointer)
    let a = Rc::new(String::from("hello rust"));
    println!(
        "Count of reference after creation: a ={}",
        Rc::strong_count(&a)
    );

    {
        let _b = Rc::clone(&a);
        println!("Count after cloning to b = {}", Rc::strong_count(&a));

        {
            let _c = Rc::clone(&a);
            println!("Count after cloning to c = {}", Rc::strong_count(&a));
        } // c goes out of scope
        println!("Count after c is dropped = {}", Rc::strong_count(&a));
    } // all reference to a dropped
    println!(
        "ref count after all clone are dropped = {}",
        Rc::strong_count(&a)
    );
    // Rc<T> Shared ownership
    let var1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating var1 = {}", Rc::strong_count(&var1));

    let b = Cons(3, Rc::clone(&var1));
    println!("count after creating b = {}", Rc::strong_count(&var1));

    {
        let c = Cons(4, Rc::clone(&var1));
        println!("count after creating c = {}", Rc::strong_count(&var1));
    }

    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&var1)
    );

    // 3.0 arc
    let dog = Arc::new(String::from("Ralf"));
    let dog_clone = Arc::clone(&dog);

    let handle = thread::spawn(move || {
        println!("From thread: {}", dog_clone);
    });
    println!("From main: {}", dog);
    handle.join().unwrap();
} // Automatically prints drop message here
