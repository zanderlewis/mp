# Mersenne Prime Checker
Check an exponent to see if it is a Mersenne Prime.

A Mersenne Prime is a prime number that is one less than a power of two. The formula for a Mersenne Prime is 2^p - 1 where p is a prime number.

This program will check if a number is prime and then check if it is a Mersenne Prime. If the number is a Mersenne Prime, it will output the number and the exponent. If the number is not a Mersenne Prime, it will output that the number is not a Mersenne Prime.

## Supported GPUs
- All GPUs that support OpenCL

## Installation
1. Clone the repository: `git clone https://github.com/zanderlewis/mp.git`
2. Run the program: `cargo run -- -h`

## To Do
- [ ] Support with GIMPS
- [ ] Create tests
- [ ] Add more documentation
