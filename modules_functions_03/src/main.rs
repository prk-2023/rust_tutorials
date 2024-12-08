/*
 * modules
 * pub mod to call the module with its filename with out rs extension
 * public functions inside the module can be called using ::
 * if the functions are nested inside submodules use the pattern
 *  mymodule::sub_module::some_func()
 *
 */

use std::io;
pub mod physics;

// When modules are in other sub-directories, we use th #[path] to the module file directory
// in our care the add.rs module file inside math folder. conatining my_add public function
#[path = "math/add.rs"]
pub mod add;

// allow attribute  to supress warming messages
#[allow(unused_variables)]
fn main() {
    println!("adding 3 and 4 we get : {}", add::my_add(3, 4));
    loop {
        // read input number from user
        let mut obj_mass = String::new();

        match io::stdin().read_line(&mut obj_mass) {
            Ok(_) => println!("Input mass of the obj: {}", obj_mass),
            Err(e) => println!("Invalid input try to pass a number"),
        }
        //convert the String to i32 :
        let obj_mass: u32 = match obj_mass.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input was not a number : pass a valid u32 ");
                continue;
            }
        };
        match obj_mass {
            0 => {
                break;
            }
            _ => println!("Error: invalid input try again"),
        }

        let acceleration: f64 = 9.8;
        let force = physics::mechanics::calculate_force(obj_mass.try_into().unwrap(), acceleration);
        println!("The force or weight of the obj is {}", force);

        let total: i32 = physics::mymath::addition::add_num(4, 5);
        println!(" calling sub modules example: add 4, 5= {}", total);

        // since add_num is re-exporeted in physics.rs it can be called directly
        let total: i32 = physics::add_num(9, 9);
        println!(" calling sub modules's re-exported add_num(9,9) {}", total);
    }
}
