#[derive(Debug)]
enum Shape {
    Circle(f64),             // Circle with radius which is a unit like struct or a f64 value
    Rectangle(f64, f64),     // Rectangle with width and height (tuple)
    ParallelGrm(f64, f64),   //parallelogram
    Triangle(f64, f64, f64), // Triangle with three sides (tuple)
}

fn match_shape(shape: Shape) {
    match shape {
        Shape::Circle(radius) => {
            println!("Circle with radius: {}", radius);
        }
        Shape::Rectangle(width, height) => {
            println!("Rectangle with width: {} and height: {}", width, height);
        }
        Shape::ParallelGrm(width, height) => {
            //parallelogram
            println!("ParalleloGram with width: {} and height: {}", width, height);
        }
        Shape::Triangle(a, b, c) if a + b > c && b + c > a && a + c > b => {
            println!("Valid Triangle with sides: {}, {}, {}", a, b, c);
        }
        Shape::Triangle(_, _, _) => {
            println!("Invalid Triangle, sides do not form a valid triangle.");
        }
    }
}

//-- ex 2 // matching complex data struct ( Option, Result and Nested Data )
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug)]
struct User {
    name: String,
    status: Status,
}

fn check_user_status(user: Option<User>) {
    match user {
        Some(User {
            name,
            status: Status::Active,
        }) => {
            println!("{} is active", name);
        }
        Some(User {
            name,
            status: Status::Inactive,
        }) => {
            println!("{} is inactive", name);
        }
        None => println!("No user provided"),
    }
}

fn check_result(result: Result<i32, String>) {
    match result {
        Ok(val) if val > 0 => println!("Success with positive value: {}", val),
        Ok(val) => println!("Success with non-positive value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let parallelgrm = Shape::ParallelGrm(10.0, 20.0);
    let triangle_valid = Shape::Triangle(3.0, 4.0, 5.0);
    let triangle_invalid = Shape::Triangle(1.0, 1.0, 10.0);

    match_shape(circle);
    match_shape(rectangle);
    match_shape(parallelgrm);
    match_shape(triangle_valid);
    match_shape(triangle_invalid);

    // Example 2
    let active_user = Some(User {
        name: "Alice".to_string(),
        status: Status::Active,
    });

    let inactive_user = Some(User {
        name: "Bob".to_string(),
        status: Status::Inactive,
    });

    let error_result: Result<i32, String> = Err("Something went wrong".to_string());
    let success_result: Result<i32, String> = Ok(42);

    check_user_status(active_user);
    check_user_status(inactive_user);
    check_user_status(None);

    check_result(error_result);
    check_result(success_result);
}
