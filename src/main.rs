use num_bigint::BigUint;
use std::env;

mod test_prime;

use test_prime::{lucas_lehmer, is_prp};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--ll" | "-l" => { // Lucas-Lehmer test
                if args.len() != 3 {
                    eprintln!("Usage: {} --ll <exponent>", args[0]);
                    return;
                }
                let exponent: u128 = match args[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Please enter a valid number for the exponent.");
                        return;
                    }
                };
                println!("Checking if 2^{} - 1 is a Mersenne prime...", exponent);
                if lucas_lehmer(exponent) {
                    let m_prime = (&<BigUint as num_traits::One>::one() << exponent) - 1u32;
                    println!("2^{} - 1 is a Mersenne prime: {}", exponent, m_prime);
                } else {
                    println!("2^{} - 1 is not a Mersenne prime.", exponent);
                }
            },
            "--prp" | "-p" => { // Probable prime test
                if args.len() < 3 || args.len() > 4 {
                    eprintln!("Usage: {} --prp <number> [<base>]", args[0]);
                    return;
                }
                let number = match BigUint::parse_bytes(args[2].as_bytes(), 10) {
                    Some(num) => num,
                    None => {
                        eprintln!("Please enter a valid number.");
                        return;
                    }
                };
                let base: u128 = if args.len() == 4 {
                    match args[3].parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Please enter a valid number for the base.");
                            return;
                        }
                    }
                } else {
                    2 // Default base
                };
                println!(
                    "Checking if {} is a probable prime with base {}...",
                    number, base
                );
                if is_prp(&number, base) {
                    println!("{} is a probable prime with base {}.", number, base);
                } else {
                    println!("{} is not a probable prime with base {}.", number, base);
                }
            },
            _ => {
                eprintln!("Usage: {} --ll <exponent> | --prp <number> [<base>]", args[0]);
            }
        }
    } else {
        eprintln!("Usage: {} --ll <exponent> | --prp <number> [<base>]", args[0]);
    }
}