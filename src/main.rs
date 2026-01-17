use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
    str::FromStr,
};

enum Operator {
    Add,
    Sub,
    Multiply,
    Div,
}

impl Operator {
    fn calculate(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Multiply => a * b,
            Self::Div => a / b,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorError;

impl FromStr for Operator {
    type Err = OperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Div),
            _ => Err(OperatorError),
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    stdout().flush().expect("Failed to flush stdout!");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer.trim().to_string()
}

fn main() {
    let a: f64 = input("Input first number: ")
        .parse()
        .expect("Failed to parse a number!");

    let b = input("Input second number: ")
        .parse()
        .expect("Failed to parse a number!");

    let operation = input("Input operation (+,-,*,/): ")
        .parse::<Operator>()
        .expect("Failed to parse a operator!");

    let result = operation.calculate(a, b);
    println!("{a}{operation}{b}={result}");
}

