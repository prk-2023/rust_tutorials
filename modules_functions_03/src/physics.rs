/* physics module
 *
 * This function can be used in the caller functions as
 *  use physics;  physics::calculate_energy()...
 */
pub fn calculate_energy(mass: f64, vel: f64) -> f64 {
    0.5 * mass * vel
}

// sub-modules inside physics
pub mod mechanics {
    // mechanics sub-module
    pub fn calculate_force(mass: f64, accel: f64) -> f64 {
        mass * accel
    }
}
// sub-module nesting..
pub mod mymath {
    pub mod addition {
        pub fn add_num(x: i32, y: i32) -> i32 {
            x + y
        }
    }
}
