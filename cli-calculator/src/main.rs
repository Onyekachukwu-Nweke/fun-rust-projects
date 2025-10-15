mod ops;

use clap::Parser;
use ops::{add, sub, mul, div};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// operand
    op: String,

    /// argument 1
    arg1: f64,

    /// argument 1
    arg2: f64,
}

fn main() {
    let args = Args::parse();

    let op = args.op;
    let lhs = args.arg1;
    let rhs = args.arg2;

    let result = match op.as_str() {
        "add" | "+" => add(lhs, rhs),
        "sub" | "-" => sub(lhs, rhs),
        "mul" | "*" => mul(lhs, rhs),
        "div" | "/" => div(lhs, rhs),
        _ => {
            println!("Unsupported operation!");
            return;
        }
    };

    println!("Result: {}", result);
}
