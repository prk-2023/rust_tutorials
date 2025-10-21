// Multiple trait bounds:  This allows you to specify that a generic type (T) must implement more
// then one trait to be used in a function or a struct definition:
// A common thing when we need a Generic Type `T` to support more a combination of behaviour
// defined by different traits:
// Synctax:
// fn my_function<T: Trait_A + Trait_B> {
//     //function body
// }
// or using `where` clause ( this is recommended for clarity )
// fn my_function<T, U>(item_t: T, item_u: U)
// where
//    T: Trait_A + Trait_B // T must implement Trait_A AND Trait_B
//    U: Trait_C + Trait_D // U mist implement Trait_C AND Trait_D
//  {
//      // function body
//  }

// Step 1: Define Traits and strut
// Using builtin  traits
// - std::fmt::Debug
// - std::clone::Clone
// and simple struct that derives them

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Appliance {
    name: String,
    power_watts: u32,
}

// Step 2: implement Fun with Multiple trait bounds:
fn print_and_clone<T>(item: &T) -> T
where
    T: std::fmt::Debug + std::clone::Clone,
{
    // Use the Debug trait (via the {:?} format specifier)
    println!("Original item: {:?}", item);

    // Use the Clone trait to create a copy
    let cloned_item = item.clone();

    println!("Cloned item created successfully.");

    // Return the clone
    cloned_item
}
// step 3: call the function:
fn main() {
    let original_fridge = Appliance {
        name: String::from("Smart Fridge"),
        power_watts: 150,
    };

    println!("\n--- Function Call ---");

    // Call the function, it works because Appliance implements both Debug and Clone.
    let mut cloned_fridge = print_and_clone(&original_fridge);

    println!("--- Results ---");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);

    // Verify they are separate objects by modifying the clone
    let _ = cloned_fridge.name.push_str(" (Modified)");

    println!("\nAfter modification check (Original should be unchanged):");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);
}

/*
 *
use std::fmt::Debug;
use std::clone::Clone;

#[derive(Debug, Clone)]
struct Appliance {
    name: String,
    power_watts: u32,
}

// This function requires T to implement both Debug AND Clone.
fn print_and_clone<T>(item: &T) -> T
where
    T: Debug + Clone,
{
    // Use the Debug trait (via the {:?} format specifier)
    println!("Original item: {:?}", item);

    // Use the Clone trait to create a copy
    let cloned_item = item.clone();

    println!("Cloned item created successfully.");

    // Return the clone
    cloned_item
}

fn main() {
    let original_fridge = Appliance {
        name: String::from("Smart Fridge"),
        power_watts: 150,
    };

    println!("\n--- Function Call ---");

    // Call the function, it works because Appliance implements both Debug and Clone.
    let cloned_fridge = print_and_clone(&original_fridge);

    println!("\n--- Results ---");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);

    // Verify they are separate objects by modifying the clone
    // NOTE: This modification happens on the *cloned_fridge* which is mutable.
    let mut cloned_fridge = cloned_fridge; // Re-bind as mutable
    cloned_fridge.name.push_str(" (Modified)");

    println!("\nAfter modification check (Original should be unchanged):");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);
}
 *
 */
