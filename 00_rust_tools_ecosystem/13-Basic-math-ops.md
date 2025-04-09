# Mathematical Operations

In Rust, the `f64` and `f32` types provide several common mathematical operations. 
Many of these operations are available through methods such as `powi` (for integer powers), as well as 
through the `f64` and `f32` standard library. 

Here's a list of some common mathematical operations available for floating-point numbers in Rust:

### 1. **Exponentiation**
   - `powi(exponent: i32)` - Raises the number to the power of the given integer exponent.
   - `powf(exponent: f64)` - Raises the number to the power of a floating-point exponent.

   Example:
   ```rust
   let x = 2.0;
   let result = x.powi(3);  // 2^3 = 8.0
   ```

### 2. **Square Root**
   - `sqrt()` - Calculates the square root of a number.
   
   Example:
   ```rust
   let x = 16.0;
   let result = x.sqrt();  // √16 = 4.0
   ```

### 3. **Logarithms**
   - `ln()` - Calculates the natural logarithm (base `e`) of a number.
   - `log(base: f64)` - Calculates the logarithm of the number with the given base.

   Example:
   ```rust
   let x = 2.718;
   let result_ln = x.ln();    // ln(2.718) ≈ 1.0 (natural log)
   let result_log = x.log(10.0); // log(10)(2.718) ≈ 0.4343
   ```

### 4. **Trigonometric Functions**
   - `sin()` - Sine of the number (in radians).
   - `cos()` - Cosine of the number (in radians).
   - `tan()` - Tangent of the number (in radians).
   - `asin()` - Arcsine (inverse sine).
   - `acos()` - Arccosine (inverse cosine).
   - `atan()` - Arctangent (inverse tangent).

   Example:
   ```rust
   let x = std::f64::consts::PI / 2.0;  // 90 degrees in radians
   let sine = x.sin();  // sin(π/2) = 1.0
   let cosine = x.cos();  // cos(π/2) = 0.0
   ```

### 5. **Rounding Functions**
   - `round()` - Rounds the number to the nearest integer.
   - `floor()` - Rounds the number down to the nearest integer.
   - `ceil()` - Rounds the number up to the nearest integer.
   - `trunc()` - Truncates the decimal part, keeping only the integer part.

   Example:
   ```rust
   let x = 2.7;
   let rounded = x.round();  // 3.0
   let floored = x.floor();  // 2.0
   let ceiled = x.ceil();    // 3.0
   let truncated = x.trunc(); // 2.0
   ```

### 6. **Absolute Value**
   - `abs()` - Returns the absolute value of the number.
   
   Example:
   ```rust
   let x = -3.5;
   let result = x.abs();  // 3.5
   ```

### 7. **Maximum and Minimum**
   - `max(other: f64)` - Returns the maximum of the two numbers.
   - `min(other: f64)` - Returns the minimum of the two numbers.
   
   Example:
   ```rust
   let x = 5.0;
   let y = 10.0;
   let maximum = x.max(y);  // 10.0
   let minimum = x.min(y);  // 5.0
   ```

### 8. **Random Numbers**
   - While not a built-in part of `f64`, you can use the `rand` crate to generate random floating-point numbers.

   Example with `rand` crate:
   ```rust
   use rand::Rng;
   let mut rng = rand::thread_rng();
   let random_number: f64 = rng.gen_range(0.0..10.0);  // Random number between 0 and 10
   ```

### 9. **Comparison**
   - `is_nan()` - Returns `true` if the number is NaN (Not-a-Number).
   - `is_infinite()` - Returns `true` if the number is infinite.
   - `is_finite()` - Returns `true` if the number is finite.

   Example:
   ```rust
   let x = std::f64::NAN;
   let is_nan = x.is_nan();  // true
   ```

### 10. **Floating-Point Constants**
   Rust provides constants for well-known mathematical values:

   - `std::f64::PI` - The constant π (Pi).
   - `std::f64::E` - The constant e (Euler's number).
   - `std::f64::INFINITY` - Represents positive infinity.
   - `std::f64::NEG_INFINITY` - Represents negative infinity.
   - `std::f64::NAN` - Represents "Not a Number".

   Example:
   ```rust
   let pi = std::f64::PI;  // 3.141592653589793
   let e = std::f64::E;    // 2.718281828459045
   ```

### 11. **Conversion**
   - You can convert between types like `i32`, `f64`, and `f32` using casting.
   
   Example:
   ```rust
   let x: f64 = 2.5;
   let y: i32 = x as i32;  // Casting f64 to i32 (truncates decimal part)
   ```

### 12. **Factorial (Manually)**
   Rust doesn't provide a built-in factorial function, but you can easily implement it yourself for integers:
   
   Example:
   ```rust
   fn factorial(n: u64) -> u64 {
       (1..=n).product()
   }

   let result = factorial(5);  // 5! = 120
   ```

These are some of the common mathematical operations and constants available in Rust's standard library for
floating-point numbers. 

You can perform most mathematical operations using these methods, and for more advanced mathematical 
functions, you can also explore crates like `num`, `nalgebra`, or `rust-num`.


