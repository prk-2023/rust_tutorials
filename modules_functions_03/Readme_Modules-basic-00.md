# Rust module Basics:


## **What are modules in Rust?**

    - In Rust, a module is a way to organize code into a logical unit. 
    - It's a collection of functions, variables, and other items that are related to each other. 
    - Modules are used to group related code together, making it easier to manage and reuse.

## **Declaring modules**
    
    - Declare a module in Rust, use the `mod` keyword followed by the name of the module. 
    For example:
    ```rust
        mod electric_functions  {
            // code related to electric_functions goes here
        }
    ```

## **Module hierarchy**

    - Modules can be nested inside each other, creating a hierarchy of modules. For example:
    ```rust
        mod atom {
            mod physical {
                // code related to physical properties  goes here
            } 
            mod chemical {
                // code related to chemical  goes here
            }
        }
    ```

## **Module imports**

    To use a module, you need to import it into your code. 
    You can do this using the `use` keyword. For example:
    ```rust
        use atom::chemical;
    ```

    This imports the `chemical` module from the `atom` module.

## **Module exports**
    
    By default modules and its functions are private and are required to make them public, i.e
    To make a module or its contents available to other modules, you need to export it. 
    You can do this using the `pub` keyword. For example:
    
    ```rust 
        pub mod electric_functions {
            // code related to physics goes here
        }
    ```

    This makes the `electric_functions` module available to other modules.

### **Example: Physics module**

    Let's create a `physics` module that contains functions related to physics. 
    We'll create a `mechanics` module inside the `physics` module that contains funs related to mechanics.

    ```Rust 
        // physics.rs 
        pub mod physics {
            pub mod mechanics {
                pub fn calculate_force(mass: f64, acceleration: f64) -> f64 {
                    mass * acceleration
                }
            }
        pub fn calculate_energy(mass: f64, velocity: f64) -> f64 {
            0.5 * mass * velocity * velocity
        }
    }
    ```

In this example, we've created a `physics` module that contains a `mechanics` module. 
The `mechanics` module contains a function `calculate_force` that calculates the force applied to an object 
given its mass and acceleration. 

The `physics` module also contains a function `calculate_energy` that calculates the energy of an object 
given its mass and velocity.

### **Using the physics module**

    To use the `physics` module, we need to import it into our code. 
    We can do this using the `use` keyword.
    ```rust 
    // main.rs 
    use physics::mechanics;

    fn main() {
        let mass = 10.0;
        let acceleration = 5.0;
        let force = mechanics::calculate_force(mass, acceleration);
        println!("Force: {}", force);
    }
    ```

we've imported the `mechanics` module from the `physics` module and used its `calculate_force` function to 
calculate the force applied to an object.

This is a basic example of how modules work in Rust. Modules are a powerful tool for organizing code and 
making it reusable.
