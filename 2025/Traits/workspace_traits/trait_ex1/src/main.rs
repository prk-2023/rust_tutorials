// Trait for human readable of themselfs
pub trait Describable {
    // about is the default method, this can be overridden by the types that impl this trait.
    fn about(&self) -> String {
        String::from("From default method: Trait Example ( allowed to override by type )")
    }

    fn describe(&self) -> String;
}

//Implementing the above trait on struct
pub struct MathProblem {
    pub question: String,
    pub difficulty: u8,
}

impl Describable for MathProblem {
    fn about(&self) -> String {
        String::from(
            "Override Default method:about => MathProblem with Question type and difficulty level",
        )
    }
    fn describe(&self) -> String {
        format!(
            "Math Problem (Level {}): {}",
            self.difficulty, self.question
        )
    }
} // this make `MathProblem` to have the behaviour defined by trait `Describable`.

// Usage of the above trait behaviour:
fn main() {
    let problem = MathProblem {
        question: String::from("What is the derivative of x^2 ?"),
        difficulty: 1,
    };

    println!("{}", problem.about());
    println!("{}", problem.describe());
}
