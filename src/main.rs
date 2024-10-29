use clap::{Arg, Command};
use num_bigint::BigUint;

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
            Arg::new("memory")
                .short('m')
                .long("memory")
                .action(clap::ArgAction::SetTrue)
                .help("Enables the use of a file to lessen the load on memory"),
        )
        .arg(
            Arg::new("from_list")
                .short('f')
                .long("from-list")
                .num_args(1)
                .conflicts_with("generate")
                .help("Reads numbers from a file and uses them for the tests"),
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
                if matches.get_flag("output") {
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
    } 
    // Handle Lucas-Lehmer Test
    else if matches.get_flag("ll") {
        let use_memory = matches.get_flag("memory");
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
                match lucas_lehmer(number, use_memory) {
                    Ok(_result) => {
                        print!("");
                    }
                    Err(e) => eprintln!("Error testing {}: {}", number, e),
                }
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
                match lucas_lehmer(number, use_memory) {
                    Ok(_result) => {
                        print!("");
                    }
                    Err(e) => eprintln!("Error testing {}: {}", number, e),
                }
            }
        } else {
            eprintln!("No numbers provided for Lucas-Lehmer test.");
        }
    } 
    // Handle Probable Prime Test
    else if matches.get_flag("prp") {
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
                println!(
                    "{}: {}",
                    number,
                    if is_prp(&BigUint::from(number), 2) {
                        "Probably prime"
                    } else {
                        "Probably not prime"
                    }
                );
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
                println!(
                    "{}: {}",
                    number,
                    if is_prp(&BigUint::from(number), 2) {
                        "Probably prime"
                    } else {
                        "Probably not prime"
                    }
                );
            }
        } else {
            eprintln!("No numbers provided for Probable Prime test.");
        }
    } else {
        eprintln!("No action specified. Use -l/--ll, -p/--prp, or -g/--generate.");
    }
}