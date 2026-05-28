# Integer Overflow:

Integer overflow happens when a program tries to store a number larger than its allocated memory can hold, 
causing the value to "wrap around" to the minimum possible number. 

It is similar to a car's odometer hitting 999,999 miles and rolling over to 000,000.

Rust handles integer overflow differently depending on whether you are compiling in **Debug** mode or 
**Release** mode. This design choice balances safety during development with performance in production.

Below is how Rust behaves in both scenarios, along with how you can customize that behavior.

---

## 1. Debug Mode (`dev` profile)

In debug builds, Rust prioritizes safety and catching bugs early.

* **Behavior:** The compiler injects runtime checks for integer overflow.
* **Result:** If an overflow occurs, the program will **panic** at runtime and terminate.

```rust
// In Debug Mode
let mut x: u8 = 255;
x += 1; // 💥 Program panics: "attempt to add with 'overflow'"

```

---

## 2. Release Mode (`release` profile)

In release builds, Rust prioritizes speed and efficiency. Runtime checks are removed because they can
significantly slow down math-heavy applications.

* **Behavior:** The compiler omits the overflow checks. Instead, it allows the value to wrap around using
  **two's complement wrap-around**.
* **Result:** The program does *not* panic. Instead, values that exceed their maximum capacity wrap
  around to their minimum possible value.

```rust // In Release Mode let mut x: u8 = 255; x += 1; // No panic! x smoothly wraps around and becomes
0

```

> ⚠️ **Important Note:** In Rust, relying on release-mode wrapping is not considered "undefined behavior"
> (unlike in C or C++), but it *is* considered a program error if you didn't intend for it to happen.

---

## Summary Comparison

| Feature | Debug Mode (`cargo run`) | Release Mode (`cargo run --release`) |
| --- | --- | --- |
| **Overflow Checks** | Enabled | Disabled |
| **Program Action** | **Panics** and exits | **Wraps around** (Two's complement) |
| **Performance Impact** | Slight overhead due to checks | Maximum performance |
| **Primary Goal** | Catch bugs during development | Speed and optimization in production |

---

## 3. How to Explicitly Control Overflow

If you *want* a specific behavior regardless of the build profile, `std` provides explicit primitive methods
for integers. You don't have to guess what the compiler will do.

### Wrapping Methods

Forces the value to wrap around, exactly like release mode.

```rust let x = 255u8.wrapping_add(1); // x is 0 in BOTH debug and release

```

### Checked Methods

Returns an `Option::None` if an overflow occurs, allowing you to handle the error gracefully without
panicking.

```rust match 255u8.checked_add(1) { Some(val) => println!("Result: {}", val), None => println!("Overflow
occurred!"), // This will execute }

```

### Overflowing Methods

Returns a tuple containing the wrapped result and a boolean indicating whether an overflow occurred.

```rust let (result, overflowed) = 255u8.overflowing_add(1); // result = 0, overflowed = true

```

### Saturating Methods

Clamps the value to the type's maximum or minimum bounds.

```rust let x = 255u8.saturating_add(1); // x stays 255 (the maximum for u8)

```

---

## 4. Overriding Defaults via `Cargo.toml`

If you want to force debug builds to wrap, or force release builds to panic on overflow, you can configure
the `overflow-checks` flag in your `Cargo.toml`:

```toml [profile.release] overflow-checks = true # Force release mode to panic on overflow

[profile.dev] overflow-checks = false # Force debug mode to wrap instead of panicking

```
