use std::{
    fmt::Display,
    io::{self, Write},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

trait Calculator {
    fn calculate(&self, a: f64, b: f64) -> f64;
}

enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Power,
}

impl Calculator for Operator {
    fn calculate(&self, a: f64, b: f64) -> f64 {
        match self {
            Operator::Add => a.add(b),
            Operator::Substract => a.sub(b),
            Operator::Multiply => a.mul(b),
            Operator::Divide => a.div(b),
            Operator::Power => a.powf(b),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Substract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Power => write!(f, "^"),
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
            "-" => Ok(Operator::Substract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "^" => Ok(Operator::Power),
            _ => Err(OperatorError),
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
    buffer.trim().to_string()
}

fn main() {
    let a: f64 = input("Input first number: ")
        .parse()
        .expect("Failed to parse a number!");

    let operation = input("Input operation (+,-,*,/,^): ")
        .parse::<Operator>()
        .expect("Failed to parse a operator!");

    let b: f64 = input("Input second number: ")
        .parse()
        .expect("Failed to parse a number!");

    let result = operation.calculate(a, b);
    println!("{a} {operation} {b} = {result}");
}
