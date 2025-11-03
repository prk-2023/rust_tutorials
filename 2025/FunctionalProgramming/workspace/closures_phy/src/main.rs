fn main() {
    // 1. By reference (&T) - immutable borrow
    let velocity = 15.0; // meters per second
    let time = 10.0; // seconds
    let calculate_distance = || {
        let distance = velocity * time; // Distance = velocity * time
        println!("Distance traveled: {} meters", distance);
    };

    calculate_distance();
    println!("Velocity is still accessible: {} m/s", velocity); // velocity is still usable here
    println!("The closure only borrows velocity immutably, not taking ownership");

    // 2. By mutable reference (&mut T) - mutable borrow
    let mut mass = 50.0; // kilograms
    let mut update_mass = || {
        mass += 5.0; // Modify mass (adding 5 kg)
        println!("Updated mass inside closure: {} kg", mass);
    };
    update_mass();
    // Try access mass:
    // below print happens after the closure, so the borrow is no longer active
    // `mass` is accessible again and its value is 55.0
    // Note In rust When a closure mutable borrows a variable you can'nt use that variable
    // elesewhere  while the closure is still running, but after the closure ends the borrow is
    // released and the variable can be accessed again.
    println!("Mass after cause closure: {}", mass);

    // Let's demonstrate mutable reference properly
    let mut velocity_final = 0.0; // m/s
    {
        let mut update_velocity = || velocity_final = velocity * 1.2; // Increase velocity by 20%
        update_velocity();
    } // mutable borrow ends here
    println!("Final velocity after update: {} m/s", velocity_final); // Now we can use velocity_final

    // 3. By value (T) - takes ownership
    let initial_position = 0.0; // meters
    let position_fn = move || {
        let final_position = initial_position + 10.0; // Assume we move 10 meters
        println!("Object's final position: {} meters", final_position);
        // initial_position is moved into the closure
    };
    position_fn();
    // println!("initial_position: {}", initial_position); // Error - ownership moved to closure

    // Demonstrating move with primitive types (Copy trait)
    let force = 100.0; // Newtons
    let take_force = move || {
        println!("Applied force: {} N", force);
    };
    take_force();
    println!("force is still accessible: {} N", force); // Works because f64 is Copy

    // Complex example: Demonstrating physics calculations with vectors and forces
    let mass1 = 10.0; // kg
    let mass2 = 20.0; // kg
    let gravitational_constant = 6.67430e-11; // m^3 kg^-1 s^-2
    let distance_between_masses: f32 = 5.0; // meters

    // By reference: Calculate gravitational force (F = G * (m1 * m2) / r^2)
    let calculate_gravitational_force = || {
        let force = gravitational_constant * (mass1 * mass2) / distance_between_masses.powi(2);
        println!("Gravitational force: {} N", force);
    };
    calculate_gravitational_force();
    println!("Mass 1 is still accessible: {} kg", mass1);
    println!("Mass 2 is still accessible: {} kg", mass2);

    // By mutable reference: Update distance and recalculate force
    let mut distance: f32 = 5.0;
    let mut update_distance = || {
        distance += 2.0; // Increase distance by 2 meters
        let updated_force = gravitational_constant * (mass1 * mass2) / distance.powi(2);
        println!("Updated gravitational force: {} N", updated_force);
    };
    update_distance();
    // println!("Distance after closure: {} meters", distance); // Error: distance is mutably borrowed

    // By value: Move masses into closure and calculate force again
    let mass1_clone = mass1;
    let mass2_clone = mass2;
    let calculate_force = move || {
        let force = gravitational_constant * (mass1_clone * mass2_clone) / distance.powi(2);
        println!("Force with moved masses: {} N", force);
    };
    calculate_force();
    // println!("mass1_clone: {}", mass1_clone); // Error - mass1_clone moved

    demonstrate_closure_types();
}

fn demonstrate_closure_types() {
    let time = 5.0;

    // Fn closure - borrows immutably
    let calculate_speed = || {
        let speed = 50.0 / time; // speed = distance/time
        println!("Speed: {} m/s", speed);
    };

    let mut velocity = 100.0; // m/s

    // FnMut closure - borrows mutably
    let mut increase_velocity = || {
        velocity += 10.0; // Increase velocity by 10 m/s
        println!("Increased velocity: {} m/s", velocity);
    };

    let mass = String::from("50 kg"); // Example of ownership transfer

    // FnOnce closure - takes ownership
    let consume_mass = move || {
        println!("Consuming mass: {}", mass);
        // mass is dropped here
    };

    calculate_speed();
    increase_velocity();
    consume_mass();
    // consume_mass(); // This would error - closure can only be called once
}
