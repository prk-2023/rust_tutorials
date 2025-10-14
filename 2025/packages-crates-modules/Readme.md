# Packages, Crates, Modules:
---
## Understand the Module system:

Rust's Module system helps you organize code as it grows:

1. Packages: A Cargo feature that lets you build, test and share crates.

    Package is a bundle of rust code. It can contain :
    - 0 or 1 *library crates*
    - 0 or more *binary crates*

    It always has a Cargo.toml file 

    Package can be thought of as a Project of App folder.

    ```bash
    $ cargo new my_project
    my_project/
    ├── Cargo.toml     # This is the package manifest
    └── src/
        └── main.rs    # This is the binary crate entry point
    ```

2. Crates: A tree of modules that produces a library or executable 

    Crates are compilation unit, there are 2 types:

    - Binary Crate: Has a `main` function and produces an executable.

    - library Crate: Has no `main` function, provides functionality to be re-used. 

    Think of crate as a deliverable what Cargo builds.

    > Every package contains at leat one crate.

3. Modules and use: let you control the organization, scope and privacy of paths.

    Module is used to organize code within a *crate*.

    You create module using 
    ```rust 
    
        mod my_module;

    ```
    Or-Inline using:
    ```rust 
    mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
    ```

    - Modules help in organizing large Codebase.
    - Controlling visibility with `pub`.
    - Creating reusable, testable components

4. Paths: A way of naming an item, such as a struct, function, or module.

    Path lets you reference items ( functions, struct, enums...) in modules. 
    * `::` is the path seperator ( like `/` in filesystem )
    * Absolute path: Starts from the Crate root.
    * Relative path: Starts from the current module.

    ex:
    `crate::utils::math::add()`
    or 
    `self::math::add()`

### Why Modules matter:

- Organize large codebase.

- Control privacy: *public* vs *private*.

- Prevent naming conflict. 

- Make code more maintainable.


### Packages and Crates:

**Key Concepts** :

- Crate: Smallest amount of code that Rust compiler considers at at time.

    - Binary Crate: Executable program 

    - Library Crate: Code intended to be used by other programs

- Package: One or more crates that provide a set of functionality 

    - Contains `Cargo.toml` file describing how to build those crates.


### Creating a package: 

```bash 
// create a package
$ cargo new my-project 
$ cd my-project 
$ tree 
my-project/
├── Cargo.toml
└── src/
    └── main.rs
```

**Cargo.toml**:
```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]

```

**src/main.rs** 
```rust 
fn main() {
    println!("Hello, world!");
}
```

### Creating a Library crate

```bash 
$ cargo new my-lib --lib
$ cd my-lib 
$ tree 
my-lib/
├── Cargo.toml
└── src/
    └── lib.rs
```

**src/lib.rs**
```rust 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

---

## Defining Modules:
--- 
### Basic Module Syntax:

Example of a restaurant:
with two modules front and back house operations

```rust 
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
``` 

This creates module tree:

```text 
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### Privacy Rules: 

- All items ( functions, methods, structs, enums, modules, constants ) are private by default.

- Use `pub` keyword to make items public.

- You can not make inner items public with out making the parent module public:

```rust 
mod front_of_house {
    pub mod hosting {
        pub  fn add_to_waitlist() { }
        fn seat_at_table() {} // this is still private.
        }
    }
}
```
### Practice: Module Basics:

Create a new Library:

```bash 
$ cargo new restaurant --lib 
```

**src/lib.rs** 
```rust 
mod restaurant {
    pub mod kitchen {
        pub mod chefs {
            pub fn cook_dish() {
                println!("Cooking delicious dish!");
            }
            
            fn prepare_ingredients() {
                println!("Preparing ingredients");
            }
        }
        
        pub mod waitstaff {
            pub fn take_order() {
                println!("Taking order");
            }
            
            pub fn serve_food() {
                println!("Serving food to customer");
            }
        }
    }
    
    pub mod management {
        pub fn hire_employee() {
            println!("Hiring new employee");
        }
        
        fn calculate_payroll() {
            println!("Calculating payroll");
        }
    }
}

// This function will be our public API
pub fn run_restaurant() {
    restaurant::kitchen::chefs::cook_dish();
    restaurant::kitchen::waitstaff::take_order();
    restaurant::management::hire_employee();
    
    // These would cause errors because they're private:
    // restaurant::kitchen::chefs::prepare_ingredients();
    // restaurant::management::calculate_payroll();
}
``` 
**src/main.rs**
```rust 
use restaurant::run_restaurant;

fn main() {
    run_restaurant();
}
```

## Paths for Referring to items:

Two types of paths to refer to items:
1. Absolute Path: Starts from crate root by using crate name or literal `crate`.

2. Relative Path: Starts from current module and uses `self`, `super` or `identifier`.


### Absolute vs Relative Paths:

```rust 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path  
    front_of_house::hosting::add_to_waitlist();
}
```
### Using `super` for Parent Modules:

```rust 
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // Call function in parent module
    }
    
    fn cook_order() {}
}
```
### Structs and Enums in Modules:

```rust 
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Private field
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer { // All variants are public if enum is public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // This would fail - seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Practice: Structs and Pricacy:

**src/lib.rs**
```rust 
mod bank {
    pub mod accounts {
        pub struct BankAccount {
            pub account_number: String,
            pub balance: f64,
            account_type: AccountType, // private field
        }
        
        pub enum AccountType {
            Checking,
            Savings,
            Business,
        }
        
        impl BankAccount {
            pub fn new(account_number: String, account_type: AccountType) -> BankAccount {
                BankAccount {
                    account_number,
                    balance: 0.0,
                    account_type,
                }
            }
            
            pub fn deposit(&mut self, amount: f64) {
                self.balance += amount;
            }
            
            pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
                if amount > self.balance {
                    Err("Insufficient funds".to_string())
                } else {
                    self.balance -= amount;
                    Ok(())
                }
            }
            
            pub fn get_account_type(&self) -> &AccountType {
                &self.account_type
            }
        }
    }
}

pub use bank::accounts::BankAccount;
pub use bank::accounts::AccountType;

pub fn demonstrate_banking() {
    let mut account = BankAccount::new(
        "12345".to_string(), 
        AccountType::Checking
    );
    
    account.deposit(1000.0);
    println!("Balance: ${}", account.balance);
    
    match account.withdraw(500.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Account type: {:?}", account.get_account_type());
    
    // These would fail - private fields:
    // println!("Type: {:?}", account.account_type);
    // account.account_type = AccountType::Savings;
}
```

## Bringing Paths into Scope:

**The `use` keyword**:

```rust 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Bring hosting into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Much shorter!
}
```
**Creating Idiomatic `use` Paths** :

```rust 
// ✅ Idiomatic - bring the parent module
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// ❌ Less idiomatic - bring the function directly
// use std::collections::HashMap::new;

```

But for structs, enums, and other items, it's idiomatic to bring the full path:

```rust 
use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
}
```

**The `as` Keyword for Aliasing** :

```rust 
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // ...
}

fn function2() -> IoResult<()> {
    // ...
}
```

**Re-Exporting with `pub use` ** :


```rust 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Re-export hosting so external code can use it directly
pub use crate::front_of_house::hosting;

// Now external code can do:
// use restaurant::hosting::add_to_waitlist;
```

**Using External Packages** :

Cargo.toml:

```toml 
[dependencies]
rand = "0.8.5"
```

src/main.rs:
```rust 
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number);
}

```
**Nested Paths for clean Imports** :

```rust 
// Instead of this:
// use std::cmp::Ordering;
// use std::io;

// Use nested paths:
use std::{cmp::Ordering, io};

// For bringing self and another item:
use std::io::{self, Write};

// Bring all public items:
use std::collections::*;

```

**Practice: Using `use` Effectively** 
src/lib.rs:
```rust 
// Library for a school management system

mod school {
    pub mod students {
        pub struct Student {
            pub name: String,
            pub grade: u8,
            student_id: u32,
        }
        
        impl Student {
            pub fn new(name: String, grade: u8) -> Student {
                static mut NEXT_ID: u32 = 1;
                let id = unsafe {
                    let id = NEXT_ID;
                    NEXT_ID += 1;
                    id
                };
                
                Student {
                    name,
                    grade,
                    student_id: id,
                }
            }
            
            pub fn get_id(&self) -> u32 {
                self.student_id
            }
        }
    }
    
    pub mod teachers {
        pub struct Teacher {
            pub name: String,
            pub subject: String,
        }
        
        impl Teacher {
            pub fn new(name: String, subject: String) -> Teacher {
                Teacher { name, subject }
            }
        }
    }
    
    pub mod courses {
        pub struct Course {
            pub name: String,
            pub credits: u8,
        }
        
        impl Course {
            pub fn new(name: String, credits: u8) -> Course {
                Course { name, credits }
            }
        }
    }
}

// Re-export the main types for easy external use
pub use school::students::Student;
pub use school::teachers::Teacher;
pub use school::courses::Course;

// Group related functionality in a clean API
pub mod api {
    use super::{Student, Teacher, Course};
    
    pub fn create_student(name: &str, grade: u8) -> Student {
        Student::new(name.to_string(), grade)
    }
    
    pub fn create_teacher(name: &str, subject: &str) -> Teacher {
        Teacher::new(name.to_string(), subject.to_string())
    }
    
    pub fn create_course(name: &str, credits: u8) -> Course {
        Course::new(name.to_string(), credits)
    }
}

// Demonstrate the module system
pub fn demonstrate_school() {
    use api::{create_student, create_teacher, create_course};
    
    let student = create_student("Alice", 10);
    let teacher = create_teacher("Mr. Smith", "Mathematics");
    let course = create_course("Algebra", 3);
    
    println!("Student: {} (ID: {})", student.name, student.get_id());
    println!("Teacher: {} - {}", teacher.name, teacher.subject);
    println!("Course: {} ({} credits)", course.name, course.credits);
}
```
## Separating Modules into Files


### Module File Structure:

As project grows, you can split modules into separate files:

```text 
restaurant/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── front_of_house.rs
    ├── front_of_house/
    │   ├── mod.rs
    │   ├── hosting.rs
    │   └── serving.rs
    └── back_of_house.rs
```

**Step-by-Step File Separation** 

Step 1: Start with everything in lib.rs

```rust 
// src/lib.rs (initial)
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
 
    pub mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn dine() {
    front_of_house::hosting::add_to_waitlist();
}
```

Step 2: Extract front_of_house in its own file:

```rust 
// src/lib.rs
mod front_of_house; // Tells Rust to load from front_of_house.rs

pub fn dine() {
    front_of_house::hosting::add_to_waitlist();
}
```

```
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
```

Step 3: Extract hosting and serving to their own files:

```rust 
// src/front_of_house.rs
pub mod hosting;  // Load from hosting.rs
pub mod serving; // Load from serving.rs
```

```
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
fn seat_at_table() {}
```

```
// src/front_of_house/serving.rs
pub fn take_order() {}
fn serve_order() {}
fn take_payment() {}
```

**Final Structure** 
```text 
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    ├── hosting.rs
    └── serving.rs
```

## Practice: Multi-file Project

Let's create a complete e-commerce system:

Project structure:

```text 
ecommerce/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── products.rs
    ├── users.rs
    ├── orders.rs
    └── inventory/
        ├── mod.rs
        ├── warehouse.rs
        └── suppliers.rs
```

**src/lib.rs**

```rust 
mod products;
mod users;
mod orders;
mod inventory;

// Re-export main functionality
pub use products::Product;
pub use users::{User, UserRole};
pub use orders::Order;
pub use inventory::{Warehouse, Supplier};

// Main public API
pub fn run_demo() {
    println!("=== E-Commerce System Demo ===");
    
    // Create some products
    let laptop = Product::new("Laptop", 999.99, 10);
    let mouse = Product::new("Mouse", 29.99, 50);
    
    // Create a user
    let mut user = User::new("alice@example.com", "Alice Smith", UserRole::Customer);
    
    // Create an order
    let mut order = Order::new(user.get_id());
    order.add_product(laptop, 1);
    order.add_product(mouse, 2);
    
    println!("Order total: ${}", order.calculate_total());
    println!("Order status: {:?}", order.get_status());
    
    // Demonstrate inventory
    let warehouse = Warehouse::new("Main Warehouse");
    println!("Warehouse: {}", warehouse.get_name());
}
```

**src/products.rs**

```rust 
#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    pub fn new(name: &str, price: f64, stock: u32) -> Self {
        Product {
            name: name.to_string(),
            price,
            stock,
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_price(&self) -> f64 {
        self.price
    }
    
    pub fn get_stock(&self) -> u32 {
        self.stock
    }
    
    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        if quantity > self.stock {
            Err("Not enough stock".to_string())
        } else {
            self.stock -= quantity;
            Ok(())
        }
    }
}
```

**src/users.rs** :
```rust 
#[derive(Debug)]
pub enum UserRole {
    Customer,
    Admin,
    Vendor,
}

pub struct User {
    email: String,
    name: String,
    role: UserRole,
    user_id: u32,
}

impl User {
    pub fn new(email: &str, name: &str, role: UserRole) -> Self {
        static mut NEXT_ID: u32 = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };
        
        User {
            email: email.to_string(),
            name: name.to_string(),
            role,
            user_id: id,
        }
    }
    
    pub fn get_id(&self) -> u32 {
        self.user_id
    }
    
    pub fn get_email(&self) -> &str {
        &self.email
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_role(&self) -> &UserRole {
        &self.role
    }
}
```

**src/orders.rs**

```rust 
use super::products::Product;

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    order_id: u32,
    user_id: u32,
    items: Vec<(Product, u32)>, // (product, quantity)
    status: OrderStatus,
}

impl Order {
    pub fn new(user_id: u32) -> Self {
        static mut NEXT_ID: u32 = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };
        
        Order {
            order_id: id,
            user_id,
            items: Vec::new(),
            status: OrderStatus::Pending,
        }
    }
    
    pub fn add_product(&mut self, product: Product, quantity: u32) {
        self.items.push((product, quantity));
    }
    
    pub fn calculate_total(&self) -> f64 {
        self.items.iter()
            .map(|(product, quantity)| product.get_price() * (*quantity as f64))
            .sum()
    }
    
    pub fn get_status(&self) -> &OrderStatus {
        &self.status
    }
    
    pub fn confirm(&mut self) {
        self.status = OrderStatus::Confirmed;
    }
}
```
src/inventory/mod.rs:

```rust 
pub mod warehouse;
pub mod suppliers;

// Re-export from submodules
pub use warehouse::Warehouse;
pub use suppliers::Supplier;
```

src/inventory/warehouse.rs:

```rust 
pub struct Warehouse {
    name: String,
    location: String,
}

impl Warehouse {
    pub fn new(name: &str) -> Self {
        Warehouse {
            name: name.to_string(),
            location: "Unknown".to_string(),
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn set_location(&mut self, location: &str) {
        self.location = location.to_string();
    }
}
```
src/inventory/suppliers.rs:
```rust 
pub struct Supplier {
    name: String,
    contact_email: String,
}

impl Supplier {
    pub fn new(name: &str, email: &str) -> Self {
        Supplier {
            name: name.to_string(),
            contact_email: email.to_string(),
        }
    }
}
```

src/main.rs 
```rust 
use ecommerce::run_demo;

fn main() {
    run_demo();
}
```

Cargo.toml:
```toml 
[package]
name = "ecommerce"
version = "0.1.0"
edition = "2021"

[dependencies]
```
---

## Advanced Module Patterns

### Module Organization Strategies

**1. Feature-based Organization**
```
src/
├── lib.rs
├── authentication/
│   ├── mod.rs
│   ├── users.rs
│   └── sessions.rs
├── payments/
│   ├── mod.rs
│   ├── credit_cards.rs
│   └── invoices.rs
└── products/
    ├── mod.rs
    ├── catalog.rs
    └── reviews.rs
```

**2. Layer-based Organization**
```
src/
├── lib.rs
├── domain/           // Business logic
│   ├── mod.rs
│   ├── entities.rs
│   └── services.rs
├── infrastructure/   // External concerns
│   ├── mod.rs
│   ├── database.rs
│   └── api.rs
└── application/      // Use cases
    ├── mod.rs
    └── commands.rs
```

### Visibility Modifiers

```rust
pub(crate) fn internal_utility() {} // Visible within crate only

pub(super) fn parent_module_only() {} // Visible to parent module

mod outer {
    pub mod inner {
        pub(in crate::outer) fn outer_module_only() {} // Visible only in outer
    }
}
```

### Complex Module Example

**src/lib.rs:**
```rust
//! A comprehensive example showing advanced module patterns

mod api {
    pub mod v1 {
        pub mod users {
            pub fn get_user() -> String {
                "User data".to_string()
            }
        }
        
        pub mod products {
            pub fn get_product() -> String {
                "Product data".to_string()
            }
        }
    }
    
    pub mod v2 {
        use super::v1; // Can use sibling modules
        
        pub mod users {
            pub fn get_user_enhanced() -> String {
                format!("Enhanced: {}", super::super::v1::users::get_user())
            }
        }
    }
}

// External interface
pub use api::v2::users::get_user_enhanced;

/// A struct demonstrating various visibility patterns
pub struct Config {
    pub api_key: String,
    pub(crate) internal_setting: String, // crate-visible only
    private_setting: String, // module-visible only
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Config {
            api_key,
            internal_setting: "default".to_string(),
            private_setting: "secret".to_string(),
        }
    }
    
    pub fn get_private_setting(&self) -> &str {
        &self.private_setting
    }
}

// Module for testing private functions
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config() {
        let config = Config::new("key".to_string());
        assert_eq!(config.get_private_setting(), "secret");
    }
}
```

---

## Exercises

### Exercise 1: Library Management System
Create a library system with this structure:
```
library/
├── books/           // Book-related modules
├── members/         // Member management
├── loans/          // Book lending logic
└── reports/        // Library statistics
```

Requirements:
- Books have title, author, ISBN, and availability status
- Members can borrow and return books
- Track due dates and fines
- Generate reports on popular books

### Exercise 2: Game Character System
Build a RPG character system:
```
rpg/
├── characters/     // Base character traits
├── inventory/      // Items and equipment  
├── skills/        // Combat and magic skills
└── battles/       // Battle system
```

Requirements:
- Multiple character classes with different abilities
- Equipment that affects character stats
- Skill trees and progression
- Turn-based battle system

### Exercise 3: Blog Engine
Create a modular blog engine:
```
blog/
├── posts/         // Blog post management
├── users/         // User accounts and roles
├── comments/      // Comment system
├── tags/         // Categorization
└── api/          // REST API endpoints
```

Requirements:
- Create, edit, delete blog posts
- User authentication and authorization
- Comment moderation
- Tag-based post organization
- Search functionality

### Exercise 4: Bank Account System
Implement a secure banking system:
```
bank/
├── accounts/      // Different account types
├── transactions/  // Deposit/withdrawal logic
├── customers/     // Customer management
├── security/     // Authentication & authorization
└── reports/      // Financial reports
```

Requirements:
- Multiple account types (checking, savings, business)
- Transaction history and balance tracking
- Customer profiles with contact information
- Role-based access control
- Audit trails for compliance

### Exercise Solutions Template

For each exercise, follow this structure:

```rust
// src/lib.rs
mod module1;
mod module2;

pub use module1::{PublicType1, PublicType2};
pub use module2::PublicType3;

/// Main public API function
pub fn run_demo() {
    // Demonstrate your system
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Your tests here
    }
}
```

This tutorial provides a comprehensive foundation for Rust's module system. 
Practice by building the exercises and experimenting with different organizational patterns to find what 
works best for your projects!
