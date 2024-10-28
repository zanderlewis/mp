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
                .action(clap::ArgAction::SetTrue)
                .help("Performs the Lucas-Lehmer test"),
        )
        .arg(
            Arg::new("prp")
                .short('p')
                .long("prp")
                .action(clap::ArgAction::SetTrue)
                .help("Performs the Probable Prime test"),
        )
        .arg(
            Arg::new("from_list")
                .short('f')
                .long("from-list")
                .num_args(1)
                .conflicts_with("generate")
                .help("Reads numbers from a file and uses them for the Lucas-Lehmer test"),
        )
        .arg(
            Arg::new("number")
                .help("Number(s) for the test")
                .num_args(1..)
                .required_unless_present("generate")
                .conflicts_with("generate"),
        )
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .num_args(2)
                .value_names(&["START", "END"])
                .help("Generates all primes in the range from START to END"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .num_args(1)
                .help("Output file for generated primes"),
        )
        .get_matches();

    if matches.contains_id("generate") {
        let mut values = matches.get_many::<String>("generate").unwrap();
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
                if matches.contains_id("output") {
                    let filename = matches.get_one::<String>("output").unwrap();
                    write_primes_to_file(&p, filename).expect("Failed to write primes to file");
                } else {
                    for prime in p {
                        println!("{}", prime);
                    }
                }
            }
            Err(e) => eprintln!("Error generating primes: {}", e),
        }
    } else if matches.contains_id("ll") || matches.contains_id("prp") {
        if matches.contains_id("from_list") {
            let filename = matches.get_one::<String>("from_list").unwrap();
            println!("Reading numbers from file {}...", filename);
            let contents = std::fs::read_to_string(filename).expect("Failed to read file");
            let numbers: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
            for number_str in numbers {
                let number: u128 = match number_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Invalid number in file: {}", number_str);
                        continue;
                    }
                };
                process_number(number, &matches);
            }
        } else if let Some(numbers) = matches.get_many::<String>("number") {
            for number_str in numbers {
                let number: u128 = match number_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Please enter a valid number.");
                        continue;
                    }
                };
                process_number(number, &matches);
            }
        } else {
            eprintln!("No numbers provided.");
        }
    } else {
        eprintln!("No action specified. Use -l/--ll or -p/--prp or -g/--generate.");
    }
}

fn process_number(number: u128, matches: &clap::ArgMatches) {
    if matches.contains_id("ll") {
        println!("Checking if 2^{} - 1 is a Mersenne prime...", number);
        if lucas_lehmer(number).unwrap() {
            let m_prime = (BigUint::from(1u32) << number) - 1u32;
            if matches.contains_id("output") {
                let filename = matches.get_one::<String>("output").unwrap();
                let mut file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(filename)
                    .expect("Failed to open file");
                writeln!(file, "{}", m_prime).expect("Failed to write to file");
            } else {
                println!("2^{} - 1 is a Mersenne prime: {}", number, m_prime);
            }
        } else {
            println!("2^{} - 1 is not a Mersenne prime.", number);
        }
    } else if matches.contains_id("prp") {
        println!("Checking if {} is a probable prime...", number);
        let big_number = BigUint::from(number);
        if is_prp(&big_number, 2) {
            println!("{} is a probable prime.", number);
        } else {
            println!("{} is not a probable prime.", number);
        }
    }
}