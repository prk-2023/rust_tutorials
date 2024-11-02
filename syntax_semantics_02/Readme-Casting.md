# Rust Type casting:
---

- Convert variables of one data type to another.

- In Rust we use "as" keyword to perform casting:

Ex:
    ```rust 
        let pi: f64 = 3.14;
        // convert the float point to integer 
        let round_pi = pi as u16;
    ```

- converting data of one type to another type manually using "as" is also called Explicit type casting.

1. Numeric Type conversion:

    ```rust 
        fn main() {
            let num: i32 = 10;
            let float_num: f64 = num as f64;
            println!("Float number: {}", float_num);

            let float_num: f64 = 10.5;
            let int_num: i32 = float_num as i32;
            println!("Integer number: {}", int_num);
        }
    ```

2. Boolean type conversion:
    Rust does not allow implicit type conversion of numeric types to boolean.
    However we can use the 'as' keyword to convert a boolean to a numeric type:

    ```rust 
        fn main() {
            let bool_val: bool = true;
            let int_val: i32 = bool_val as i32;
            println!("Integer value: {}", int_val);
        }
    ```

3. Char type conversion:
    Rust allows you to convert a char type to its corresponding ASCII values using the 'as' keyword:

    ```rust 
        fn main () {
            let char_val: chat =  'A';
            let ascii_val: u8 = char_val as u8;
            println!("ASCII value: {}", ascii_val);
        }
    ```

5. ENum  type conversion:

    Rust allows you to convert enum values to its underlying type using the 'as' keyword:

    ```rust 
        enum Color {
            red = 1,
            blue = 2,
            green = 3,
        }
        fn main () {
            let color: Color = Color::Red;
            let int_val: i32 = color as i32;
            println!("Integer value: {}", int_val);
        }
    ```

6. Reference type Conversion:

    Rust allows you to convert a "reference to a value" to a "reference to a trait object" using the 'as'
    keyword:

    ```rust 
        trait Printable {
            fn print(&self);
        }
        struct Person {
            name: String,
        }
        impl Printable for Person {
            fn print(&self) {
                println!("Name : {}", self.name);
            }
        }
        fn main () {
            let person: & Person = &Person { name: "mrRust".to_string) };
            let printable : $dyn Printable = person as &dyn Printable;
            printable.print();
        }
    ```
Recap 
---

Trait Declaration:

    trait Printable {
        fn print (&self)
    }

    - A trait is a way to define set of methods that a type can inplement.
    - In the above case we're defining a trait called "Printable" that has a single method called "print".
    - the "print" method takes a reference to "self" as an argument which means its a method that operates
      on the type that implements the trait.

    struct Person {
        name: String,
    }

Trait Implementation:

    impl Printable for Person {
        fn print (&self) {
            println! ("Name : {}", self.name);
        }
    }

    - we're implementing the "Printable" trait for the "Person" Struct.
    - This means that "Person" now has a "print" method that can be called on instance of Person.
    - The "print" method takes a reference to "self" as an argument and prints out the "name" field of the
      "Person" instance.

Main Function:
    fn main() {
        let person: &Person = &Person { name: "John".to_string() };
        let printable: &dyn Printable = person as &dyn Printable;
        printable.print();
    }

    - we have created a new instance of Person with name "John".
    - we store the reference to the Person instance in a variable "person"
    - we then casting the "person" reference to a trait object of type &dyn printable. Using the "as"
      keyword.

    - The "dyn" keyword is used to inducate that the trait object reference is dynamic, meaning that the
      actual type of the object being reference is determined at runtime.

    - We're then calling the "print" method on the "pintable" trait object reference. This will call the
      "print" method that we implemented for Person.

Concepts Included in the above example:

    - traits: Traits are ways to define a set of methods that a type can implement. They are similar to
      interfaces in CPP.
    - Trait Implementation: Implementating a trait for a type means providing a definition for each method
      in the trait.
    - Trait object: Trait objects are reference to the value that implement a trait. They are used to store
      values of different types that implement the same trait.

    - Dynamic Dispatch: Is the process of determining the actual method to call at runtime, rather then at
      compile time. This is what happens when we call the "print" method on the "printable" trait object
      reference.



