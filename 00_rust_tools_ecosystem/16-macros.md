# Macros in Rust:
---

### 🔍 **“Macros in Rust are substantially different from macros in C.”**

Rust's macros are **not just simple text substitution** like C macros. In C:

```c
#define SQUARE(x) x * x
```

This is pure **text replacement**, which can lead to issues:

```c
int result = SQUARE(1 + 2); // expands to 1 + 2 * 1 + 2 = 1 + 2 + 2 = 5 (wrong!)
```

---

### 🌳 **“They work on the syntax tree and are hence not prone to substitution issues.”**

Rust macros operate on the **abstract syntax tree (AST)**. This means:

* The Rust compiler understands the **structure** of the code inside a macro.
* There is no raw text substitution—Rust macros generate **valid, structured Rust code**, avoiding 
  problems like unintended precedence or naming conflicts.

So Rust's macro system is **safer and more powerful**.

---

### 🛠️ **“Can be used for static code generation, as they integrate nicely with the strict type system.”**

Because Rust macros understand the code structure and integrate with the type system, you can:

* Generate repetitive or boilerplate code at compile time.
* Define complex logic once and reuse it with different types or parameters.
* Still catch **type errors** at compile time, unlike C where macro expansion might produce invalid code 
  that fails silently or at runtime.

For example:

```rust
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("You called {:?}()", stringify!($name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
```

This generates two functions at compile time.

---

### ✍️ **“In addition to the built-in macros, the programmer can easily define his/her own macros.”**

Rust has built-in macros like `println!`, `vec!`, `format!`, etc.

But you can also define:

* **Declarative macros** using `macro_rules!` (simpler).
* **Procedural macros** for more advanced use, like deriving traits automatically (`#[derive(Debug)]`), attribute-like macros, or function-like macros.

---

### ✅ Summary

| Feature                     | Rust Macros                               | C Macros                         |
| --------------------------- | ----------------------------------------- | -------------------------------- |
| Based on syntax             | ✅ Yes (AST-based)                         | ❌ No (text substitution)         |
| Type-checked                | ✅ Yes                                     | ❌ No                             |
| Custom macro support        | ✅ Yes (`macro_rules!`, procedural macros) | ✅ Yes (`#define`, but less safe) |
| Compile-time generation     | ✅ Yes                                     | ✅ Yes (but unsafe and limited)   |
| Safe from substitution bugs | ✅ Yes                                     | ❌ No                             |

---

Rust macros are powerful tools for **safe, reusable, and type-aware code generation**, offering a major 
advantage over C-style macros.
