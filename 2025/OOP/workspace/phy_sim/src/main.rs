#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector3 {
    //Constructor ( similar to class constructor )
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Zero vector that is similar to class method ( function )
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    //Instance method : magniture
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z + self.z).sqrt()
    }

    // Instance method: Normalize ( returns a new unit vector )
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Self::zero()
        }
    }
    // Instance method: dot product
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Instance method: cross product
    pub fn cross(&self, other: &Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    // Mutating method: add in place
    pub fn add_assign(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
// Implementing Operator Overload:
// operator overloading lets you use operators on objects of a class, giving them special meaning and behavior
use std::ops;
//trait
impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

//Interfaces in Rust are traits: traits define behavior across different types.
// A trait defining common behavior for physical bodies.
pub trait PhysicsBody {
    // get the bodies position in 3D space.
    fn position(&self) -> Vector3;

    // Get the body's velocity
    fn velocity(&self) -> Vector3;

    // Get the body's mass
    fn mass(&self) -> f64;

    // Calculate kinetic energy (1/2 * m * v^2)
    fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass() * self.velocity().magnitude().powi(2)
    }

    // Calculate momentum (m * v)
    fn momentum(&self) -> Vector3 {
        self.velocity() * self.mass()
    }

    // Update the body's state (default implementation)
    fn update(&mut self, delta_time: f64) {
        // Basic Euler integration
        let _new_position = self.position() + self.velocity() * delta_time;
        // Default does nothing - concrete types should override if needed
    }
}

// A simple particle in physics simulation
pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    mass: f64,
    charge: f64, // Additional property specific to Particle
}

impl Particle {
    pub fn new(position: Vector3, velocity: Vector3, mass: f64, charge: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            charge,
        }
    }

    pub fn charge(&self) -> f64 {
        self.charge
    }

    // Method specific to charged particles
    pub fn lorentz_force(&self, electric_field: Vector3, magnetic_field: Vector3) -> Vector3 {
        let q = self.charge;
        let v = self.velocity;
        let e_force = electric_field * q;
        let b_force = v.cross(&magnetic_field) * q;
        e_force + b_force
    }
}
impl PhysicsBody for Particle {
    fn position(&self) -> Vector3 {
        self.position
    }

    fn velocity(&self) -> Vector3 {
        self.velocity
    }

    fn mass(&self) -> f64 {
        self.mass
    }

    fn update(&mut self, delta_time: f64) {
        // Update position based on velocity (basic integration)
        self.position = self.position + self.velocity * delta_time;
    }
}

// A rigid body with additional rotational properties
pub struct RigidBody {
    position: Vector3,
    velocity: Vector3,
    mass: f64,
    moment_of_inertia: f64, // Additional property specific to rigid bodies
    angular_velocity: f64,  // Rotation around z-axis (simplified)
}

impl RigidBody {
    pub fn new(
        position: Vector3,
        velocity: Vector3,
        mass: f64,
        moment_of_inertia: f64,
        angular_velocity: f64,
    ) -> Self {
        Self {
            position,
            velocity,
            mass,
            moment_of_inertia,
            angular_velocity,
        }
    }

    pub fn moment_of_inertia(&self) -> f64 {
        self.moment_of_inertia
    }

    pub fn angular_velocity(&self) -> f64 {
        self.angular_velocity
    }

    // Method specific to rigid bodies
    pub fn rotational_energy(&self) -> f64 {
        0.5 * self.moment_of_inertia * self.angular_velocity.powi(2)
    }
}
impl PhysicsBody for RigidBody {
    fn position(&self) -> Vector3 {
        self.position
    }

    fn velocity(&self) -> Vector3 {
        self.velocity
    }

    fn mass(&self) -> f64 {
        self.mass
    }

    // Override kinetic energy to include rotational energy
    fn kinetic_energy(&self) -> f64 {
        let translational_energy = 0.5 * self.mass * self.velocity.magnitude().powi(2);
        let rotational_energy = 0.5 * self.moment_of_inertia * self.angular_velocity.powi(2);
        translational_energy + rotational_energy
    }
}
// Polymorphhism
// A physics system that can contain different types of bodies
#[allow(dead_code)]
pub struct PhysicsSystem {
    bodies: Vec<Box<dyn PhysicsBody>>,
    gravity: Vector3,
}

impl PhysicsSystem {
    pub fn new(gravity: Vector3) -> Self {
        Self {
            bodies: Vec::new(),
            gravity,
        }
    }

    // Add any type that implements PhysicsBody (polymorphism!)
    pub fn add_body<B: PhysicsBody + 'static>(&mut self, body: B) {
        self.bodies.push(Box::new(body));
    }

    /// Calculate total kinetic energy of all bodies
    pub fn total_kinetic_energy(&self) -> f64 {
        self.bodies.iter().map(|body| body.kinetic_energy()).sum()
    }

    /// Calculate total momentum of the system
    pub fn total_momentum(&self) -> Vector3 {
        self.bodies
            .iter()
            .fold(Vector3::zero(), |acc, body| acc + body.momentum())
    }

    /// Update all bodies in the system
    pub fn update(&mut self, delta_time: f64) {
        for body in &mut self.bodies {
            // Apply gravity (simplified - assumes constant acceleration)

            // if let Some(particle) = body.as_mut().downcast_mut::<Particle>() {
            //     // Specific behavior for particles
            //     particle.velocity = particle.velocity + self.gravity * delta_time;
            // } else if let Some(rigid_body) = body.as_mut().downcast_mut::<RigidBody>() {
            //     // Specific behavior for rigid bodies
            //     rigid_body.velocity = rigid_body.velocity + self.gravity * delta_time;
            // }

            // Common update logic from trait
            body.update(delta_time);
        }
    }

    /// Display information about all bodies (dynamic dispatch)
    pub fn display_bodies(&self) {
        println!("Physics System with {} bodies:", self.bodies.len());
        println!("Total kinetic energy: {:.2} J", self.total_kinetic_energy());
        println!("Total momentum: {:?}", self.total_momentum());

        for (i, body) in self.bodies.iter().enumerate() {
            println!("\nBody {}:", i + 1);
            println!("  Position: {:?}", body.position());
            println!("  Velocity: {:?}", body.velocity());
            println!("  Mass: {:.2} kg", body.mass());
            println!("  Kinetic Energy: {:.2} J", body.kinetic_energy());
        }
    }
}
// Base trait for mathematical functions
pub trait MathematicalFunction {
    fn evaluate(&self, x: f64) -> f64;
    fn derivative(&self) -> Box<dyn MathematicalFunction>;
    fn integral(&self) -> Box<dyn MathematicalFunction>;
    fn name(&self) -> &str;
}

/// A simple linear function: f(x) = mx + b
pub struct LinearFunction {
    slope: f64,
    intercept: f64,
}

impl LinearFunction {
    pub fn new(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }
}

impl MathematicalFunction for LinearFunction {
    fn evaluate(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    fn derivative(&self) -> Box<dyn MathematicalFunction> {
        // Derivative of mx + b is m (constant function)
        Box::new(ConstantFunction::new(self.slope))
    }

    fn integral(&self) -> Box<dyn MathematicalFunction> {
        // Integral of mx + b is (m/2)x² + bx + C
        Box::new(QuadraticFunction::new(
            self.slope / 2.0,
            self.intercept,
            0.0,
        ))
    }

    fn name(&self) -> &str {
        "Linear Function"
    }
}

/// A constant function: f(x) = c
pub struct ConstantFunction {
    value: f64,
}

impl ConstantFunction {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl MathematicalFunction for ConstantFunction {
    fn evaluate(&self, _x: f64) -> f64 {
        self.value
    }

    fn derivative(&self) -> Box<dyn MathematicalFunction> {
        // Derivative of constant is zero
        Box::new(ConstantFunction::new(0.0))
    }

    fn integral(&self) -> Box<dyn MathematicalFunction> {
        // Integral of c is cx
        Box::new(LinearFunction::new(self.value, 0.0))
    }

    fn name(&self) -> &str {
        "Constant Function"
    }
}

/// A quadratic function: f(x) = ax² + bx + c
pub struct QuadraticFunction {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticFunction {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

impl MathematicalFunction for QuadraticFunction {
    fn evaluate(&self, x: f64) -> f64 {
        self.a * x * x + self.b * x + self.c
    }

    fn derivative(&self) -> Box<dyn MathematicalFunction> {
        // Derivative of ax² + bx + c is 2ax + b
        Box::new(LinearFunction::new(2.0 * self.a, self.b))
    }

    fn integral(&self) -> Box<dyn MathematicalFunction> {
        // Integral of ax² + bx + c is (a/3)x³ + (b/2)x² + cx + C
        Box::new(CubicFunction::new(
            self.a / 3.0,
            self.b / 2.0,
            self.c,
            0.0, // integration constant
        ))
    }

    fn name(&self) -> &str {
        "Quadratic Function"
    }
}

/// A cubic function: f(x) = ax³ + bx² + cx + d
pub struct CubicFunction {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl CubicFunction {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }
}

impl MathematicalFunction for CubicFunction {
    fn evaluate(&self, x: f64) -> f64 {
        self.a * x.powi(3) + self.b * x * x + self.c * x + self.d
    }

    fn derivative(&self) -> Box<dyn MathematicalFunction> {
        // Derivative of ax³ + bx² + cx + d is 3ax² + 2bx + c
        Box::new(QuadraticFunction::new(3.0 * self.a, 2.0 * self.b, self.c))
    }

    fn integral(&self) -> Box<dyn MathematicalFunction> {
        // Integral of ax³ + bx² + cx + d is (a/4)x⁴ + (b/3)x³ + (c/2)x² + dx + C
        Box::new(QuarticFunction::new(
            self.a / 4.0,
            self.b / 3.0,
            self.c / 2.0,
            self.d,
            0.0, // integration constant
        ))
    }

    fn name(&self) -> &str {
        "Cubic Function"
    }
}

/// A quartic function (composition example)
pub struct QuarticFunction {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
}

impl QuarticFunction {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
        Self { a, b, c, d, e }
    }
}

impl MathematicalFunction for QuarticFunction {
    fn evaluate(&self, x: f64) -> f64 {
        self.a * x.powi(4) + self.b * x.powi(3) + self.c * x * x + self.d * x + self.e
    }

    fn derivative(&self) -> Box<dyn MathematicalFunction> {
        Box::new(CubicFunction::new(
            4.0 * self.a,
            3.0 * self.b,
            2.0 * self.c,
            self.d,
        ))
    }

    fn integral(&self) -> Box<dyn MathematicalFunction> {
        panic!("Quartic integration not implemented for this example");
    }

    fn name(&self) -> &str {
        "Quartic Function"
    }
}
#[allow(dead_code)]
mod linear_algebra {
    /// A matrix implementation with encapsulation
    pub struct Matrix {
        rows: usize,
        cols: usize,
        data: Vec<Vec<f64>>, // Private field - encapsulation
    }

    impl Matrix {
        /// Public constructor
        pub fn new(rows: usize, cols: usize) -> Self {
            let data = vec![vec![0.0; cols]; rows];
            Self { rows, cols, data }
        }

        /// Create matrix from 2D vector (public)
        pub fn from_vec(data: Vec<Vec<f64>>) -> Result<Self, String> {
            if data.is_empty() || data[0].is_empty() {
                return Err("Matrix cannot be empty".to_string());
            }

            let rows = data.len();
            let cols = data[0].len();

            // Validate all rows have same length
            for row in &data {
                if row.len() != cols {
                    return Err("All rows must have the same length".to_string());
                }
            }

            Ok(Self { rows, cols, data })
        }

        /// Get number of rows (public getter)
        pub fn rows(&self) -> usize {
            self.rows
        }

        /// Get number of columns (public getter)
        pub fn cols(&self) -> usize {
            self.cols
        }

        /// Get element at position (mutable)
        pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f64> {
            if row < self.rows && col < self.cols {
                Some(&mut self.data[row][col])
            } else {
                None
            }
        }

        /// Get element at position (immutable)
        pub fn get(&self, row: usize, col: usize) -> Option<f64> {
            if row < self.rows && col < self.cols {
                Some(self.data[row][col])
            } else {
                None
            }
        }

        /// Set element at position
        pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
            if row >= self.rows || col >= self.cols {
                return Err("Index out of bounds".to_string());
            }
            self.data[row][col] = value;
            Ok(())
        }

        /// Matrix multiplication (private implementation detail)
        fn multiply_impl(&self, other: &Matrix) -> Result<Matrix, String> {
            if self.cols != other.rows {
                return Err("Matrix dimensions don't match for multiplication".to_string());
            }

            let mut result = Matrix::new(self.rows, other.cols);

            for i in 0..self.rows {
                for j in 0..other.cols {
                    let mut sum = 0.0;
                    for k in 0..self.cols {
                        sum += self.data[i][k] * other.data[k][j];
                    }
                    result.data[i][j] = sum;
                }
            }

            Ok(result)
        }

        /// Public matrix multiplication method
        pub fn multiply(&self, other: &Matrix) -> Result<Matrix, String> {
            self.multiply_impl(other)
        }

        /// Calculate determinant (private recursive implementation)
        fn determinant_impl(&self) -> Result<f64, String> {
            if self.rows != self.cols {
                return Err("Determinant only defined for square matrices".to_string());
            }

            match self.rows {
                1 => Ok(self.data[0][0]),
                2 => Ok(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]),
                n => {
                    let mut det = 0.0;
                    for col in 0..n {
                        let minor = self.minor(0, col)?;
                        let sign = if col % 2 == 0 { 1.0 } else { -1.0 };
                        det += sign * self.data[0][col] * minor.determinant_impl()?;
                    }
                    Ok(det)
                }
            }
        }

        /// Public determinant method
        pub fn determinant(&self) -> Result<f64, String> {
            self.determinant_impl()
        }

        /// Get minor matrix (private helper)
        fn minor(&self, row_to_remove: usize, col_to_remove: usize) -> Result<Matrix, String> {
            let mut minor_data = Vec::new();

            for i in 0..self.rows {
                if i == row_to_remove {
                    continue;
                }
                let mut new_row = Vec::new();
                for j in 0..self.cols {
                    if j == col_to_remove {
                        continue;
                    }
                    new_row.push(self.data[i][j]);
                }
                minor_data.push(new_row);
            }

            Matrix::from_vec(minor_data)
        }
    }

    /// Identity matrix factory function
    pub fn identity_matrix(n: usize) -> Matrix {
        let mut matrix = Matrix::new(n, n);
        for i in 0..n {
            matrix.set(i, i, 1.0).unwrap();
        }
        matrix
    }
}
// Main physics simulation demonstrating OOP concepts
fn main() {
    println!("=== Mathematical Functions Demonstration ===");

    // Polymorphic function collection
    let functions: Vec<Box<dyn MathematicalFunction>> = vec![
        Box::new(LinearFunction::new(2.0, 3.0)),
        Box::new(QuadraticFunction::new(1.0, -2.0, 1.0)),
        Box::new(CubicFunction::new(0.5, 0.0, -2.0, 5.0)),
    ];

    // Evaluate all functions at x = 2.0
    println!("Evaluating functions at x = 2.0:");
    for (i, func) in functions.iter().enumerate() {
        let value = func.evaluate(2.0);
        println!("  {}: {} = {:.2}", i + 1, func.name(), value);

        // Get and evaluate derivative
        let derivative = func.derivative();
        let deriv_value = derivative.evaluate(2.0);
        println!("    Derivative at x=2.0: {:.2}", deriv_value);
    }

    println!("\n=== Physics Simulation ===");

    // Create physics system
    let gravity = Vector3::new(0.0, -9.81, 0.0);
    let mut physics_system = PhysicsSystem::new(gravity);

    // Add different types of bodies (polymorphism)
    let particle = Particle::new(
        Vector3::new(0.0, 10.0, 0.0), // position
        Vector3::new(5.0, 0.0, 0.0),  // velocity
        2.0,                          // mass
        1.0e-6,                       // charge
    );

    let rigid_body = RigidBody::new(
        Vector3::new(0.0, 5.0, 0.0), // position
        Vector3::new(3.0, 0.0, 0.0), // velocity
        5.0,                         // mass
        2.0,                         // moment of inertia
        1.5,                         // angular velocity
    );

    physics_system.add_body(particle);
    physics_system.add_body(rigid_body);

    // Run simulation steps
    println!("Initial state:");
    physics_system.display_bodies();

    let delta_time = 0.1;
    for step in 1..=5 {
        println!("\n=== Simulation Step {} (Δt = {}) ===", step, delta_time);
        physics_system.update(delta_time);
        physics_system.display_bodies();
    }

    println!("\n=== Matrix Operations ===");

    // Use the linear algebra module
    use linear_algebra::{identity_matrix, Matrix};

    // Create matrices
    let a_data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

    let b_data = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];

    let matrix_a = Matrix::from_vec(a_data).unwrap();
    let matrix_b = Matrix::from_vec(b_data).unwrap();

    println!("Matrix A ({}x{}):", matrix_a.rows(), matrix_a.cols());
    println!("Matrix B ({}x{}):", matrix_b.rows(), matrix_b.cols());

    // Matrix multiplication
    match matrix_a.multiply(&matrix_b) {
        Ok(result) => {
            println!("Matrix A × B ({}x{}):", result.rows(), result.cols());
            for i in 0..result.rows() {
                for j in 0..result.cols() {
                    print!("{:.1} ", result.get(i, j).unwrap());
                }
                println!();
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Identity matrix
    let identity = identity_matrix(3);
    println!("\n3x3 Identity Matrix:");
    for i in 0..identity.rows() {
        for j in 0..identity.cols() {
            print!("{:.0} ", identity.get(i, j).unwrap());
        }
        println!();
    }
}
