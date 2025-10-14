mod inventory;
mod orders;
mod products;
mod users;

//use std::iter::Product;

//Re-Export main functionality
pub use inventory::{Supplier, Warehouse};
pub use orders::Order;
pub use products::Product;
pub use users::{User, UserRole};

//Main public API

pub fn run_demo() {
    println!("===ecom system demo===");

    //Create some products
    let laptop = Product::new("Laptop", 999.99, 10);
    let mouse = Product::new("Mouse", 29.99, 50);

    //Create a User
    let mut user = User::new("abc@xyz.com", "abc", UserRole::Customer);

    //Create an Order
    let mut order = Order::new(user.get_id());
    order.add_product(laptop, 1);
    order.add_product(mouse, 2);

    println!("Order total : ${}", order.calculate_total());
    println!("Order Status: {:?}", order.get_status());

    //Demonstrate inventory:
    let warehouse = Warehouse::new("Main Warehouse");
    println!("Warehouse: {}", warehouse.get_name());
}
