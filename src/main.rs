
use std::io::prelude::*;
use doctor_syn::{Expression};

fn main() {
    eprintln!("Doctor Syn - a computer algebra system for Rust.");

    let bufreader = std::io::BufReader::new(std::io::stdin());

    for line in bufreader.lines() {
        let line = line.unwrap();
        println!("cmd: {}", line);
        let expr : Expression = match line.parse() {
            Ok(expr) => expr,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };
        println!("expr: {}", expr);
    }
}