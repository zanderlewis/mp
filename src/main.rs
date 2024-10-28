use clap::{Arg, Command};
use num_bigint::BigUint;
use std::io::Write;

mod test_prime;
mod generate_primes;

use test_prime::{is_prp, lucas_lehmer};
use generate_primes::{generate_primes, write_primes_to_file};

fn main() {
    let matches = Command::new("Prime Checker")
        .version("1.0")
        .author("Zander Lewis <zander@zanderlewis.dev>")
        .about("Performs Lucas-Lehmer and PRP tests")
        .arg(
            Arg::new("ll")
                .short('l')
                .long("ll")
                .takes_value(false)
                .help("Performs the Lucas-Lehmer test"),
        )
        .arg(
            Arg::new("prp")
                .short('p')
                .long("prp")
                .takes_value(false)
                .help("Performs the Probable Prime test"),
        )
        .arg(
            Arg::new("from_list")
                .short('f')
                .long("from-list")
                .takes_value(true)
                .conflicts_with("generate")
                .help("Reads numbers from a file and uses them for the Lucas-Lehmer test"),
        )
        .arg(
            Arg::new("number")
                .help("Number(s) for the test")
                .takes_value(true)
                .multiple_values(true)
                .required_unless_present("generate")
                .conflicts_with("generate"),
        )
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["START", "END"])
                .help("Generates all primes in the range from START to END"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Output file for generated primes"),
        )
        .get_matches();

    if matches.is_present("generate") {
        let mut values = matches.values_of("generate").unwrap();
        let start = values
            .next()
            .unwrap()
            .parse::<u128>()
            .expect("Invalid start number");
        let end = values
            .next()
            .unwrap()
            .parse::<u128>()
            .expect("Invalid end number");
        match generate_primes(start, end) {
            Ok(p) => {
                if matches.is_present("output") {
                    let filename = matches.value_of("output").unwrap();
                    write_primes_to_file(&p, filename).expect("Failed to write primes to file");
                } else {
                    for prime in p {
                        println!("{}", prime);
                    }
                }
            }
            Err(e) => eprintln!("Error generating primes: {}", e),
        }
    } else if matches.is_present("ll") || matches.is_present("prp") {
        let numbers = matches.values_of("number").unwrap();
        for number_str in numbers {
            let number: u128 = match number_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Please enter a valid number.");
                    continue;
                }
            };
            if matches.is_present("ll") {
                if matches.is_present("from_list") {
                    let filename = matches.value_of("from_list").unwrap();
                    println!("Reading numbers from file {}...", filename);
                    let numbers = std::fs::read_to_string(filename).expect("Failed to read file");
                    let numbers: Vec<u128> = numbers
                        .lines()
                        .map(|line| line.parse().expect("Invalid number in file"))
                        .collect();
                    for number in numbers {
                        println!("Checking if 2^{} - 1 is a Mersenne prime...", number);
                        if lucas_lehmer(number).unwrap() {
                            if matches.is_present("output") {
                                let filename = matches.value_of("output").unwrap();
                                let mut file = std::fs::OpenOptions::new()
                                    .append(true)
                                    .create(true)
                                    .open(filename)
                                    .expect("Failed to open file");
                                let m_prime = (&<BigUint as num_traits::One>::one() << number) - 1u32;
                                writeln!(file, "{}", m_prime).expect("Failed to write to file");
                            } else {
                                let m_prime = (&<BigUint as num_traits::One>::one() << number) - 1u32;
                                println!("2^{} - 1 is a Mersenne prime: {}", number, m_prime);
                            }
                        } else {
                            println!("2^{} - 1 is not a Mersenne prime.", number);
                        }
                    }
                    continue;
                }
                else {
                    println!("Checking if 2^{} - 1 is a Mersenne prime...", number);
                    if lucas_lehmer(number).unwrap() {
                        let m_prime = (&<BigUint as num_traits::One>::one() << number) - 1u32;
                        println!("2^{} - 1 is a Mersenne prime: {}", number, m_prime);
                    } else {
                        println!("2^{} - 1 is not a Mersenne prime.", number);
                    }
                }
            } else if matches.is_present("prp") {
                println!("Checking if {} is a probable prime...", number);
                let big_number = BigUint::from(number);
                if is_prp(&big_number, 2) {
                    println!("{} is a probable prime.", number);
                } else {
                    println!("{} is not a probable prime.", number);
                }
            }
        }
    } else {
        eprintln!("No action specified. Use -l/--ll or -p/--prp or -g/--generate.");
    }
}