use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
}

// Implement a Debug print trait to print the Player struct:
// Do not derive the Debug trait.
struct Player {
    name: String,
    age: u32,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}
fn main() {
    let person = Person {
        name: String::from("John"),
        age: 30,
    };
    println!("{:?}", person);

    let color = Color::Green;
    println!("{:?}", color);

    let player = Player {
        name: String::from("Tilak"),
        age: 30,
    };
    println!("{}", player);
}
