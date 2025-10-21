// Similar to example 2
trait Describable {
    fn describe(&self) -> String;
}

struct CodeSnippet {
    code: String,
    language: String,
}

impl Describable for CodeSnippet {
    fn describe(&self) -> String {
        format!("Code in {}: {}", self.language, self.code)
    }
}

struct Equation {
    formula: String,
}

impl Describable for Equation {
    fn describe(&self) -> String {
        format!("Equation: {}", self.formula)
    }
}

// trait in fun parameter Use one of the below forms:
// ysing "impl Trait" syntax
fn print_description(item: &impl Describable) {
    println!("{}", item.describe());
}
// with generic syntax (or trait bound syntax)
// fn print_description<T: Describable>(item: &T) {
//     println!("{}", item.describe())
// }

fn main() {
    let code = CodeSnippet {
        code: String::from(
            "fn factorial(n: u32) -> u32 { if n == 0 { 1 } else { n * factorial(n - 1) } }",
        ),
        language: String::from("Rust"),
    };

    let eq = Equation {
        formula: String::from("E = mc^2"),
    };

    print_description(&code);
    print_description(&eq);
}
