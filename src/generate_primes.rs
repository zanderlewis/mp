use ocl::{flags, Buffer, Context, Device, Kernel, Platform, Program, Queue};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// Generates prime numbers in the range [start_n, end_n) using OpenCL for parallel processing.
///
/// # Arguments
///
/// * `start_n` - The starting number of the range.
/// * `end_n` - The ending number of the range.
///
/// # Returns
///
/// A vector containing all prime numbers within the specified range.
pub fn generate_primes(start_n: u128, end_n: u128) -> Result<Vec<u128>, Box<dyn Error>> {
    // Step 1: Initialize OpenCL
    let platform = Platform::default();
    let device = Device::first(platform)?;
    let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build()?;
    let queue = Queue::new(&context, device, None)?;

    // Step 2: Load and build the OpenCL program
    let kernel_src = r#"
    __kernel void is_prime_kernel(__global const ulong* numbers, __global ulong* results, ulong base) {
        int gid = get_global_id(0);
        ulong n = numbers[gid];
        if (n < 2) {
            results[gid] = 0;
            return;
        }
        if (n == 2) {
            results[gid] = 1;
            return;
        }
        if (n % 2 == 0) {
            results[gid] = 0;
            return;
        }

        // Fermat Primality Test: a^(n-1) mod n = 1
        ulong a = base;
        ulong result = 1;
        ulong exponent = n - 1;
        ulong power = a % n;

        while (exponent > 0) {
            if (exponent & 1) {
                // result = (result * power) % n
                result = (result * power) % n;
            }
            // power = (power * power) % n
            power = (power * power) % n;
            exponent >>= 1;
        }

        // If result == 1, then n is a probable prime
        results[gid] = (result == 1) ? 1 : 0;
    }
    "#;

    let program = Program::builder()
        .src(kernel_src)
        .devices(device)
        .build(&context)?;

    let kernel = Kernel::builder()
        .program(&program)
        .name("is_prime_kernel")
        .queue(queue.clone())
        .arg(None::<&Buffer<u64>>) // Placeholder for numbers
        .arg(None::<&Buffer<u64>>) // Placeholder for results
        .arg(2u64) // Base for Fermat Test
        .build()?;

    // Step 3: Prepare data
    let range: Vec<u128> = (start_n..end_n).collect();
    let range_len = range.len();

    // Convert to u64, ensuring values fit
    let numbers: Vec<u64> = range.iter().map(|&n| n as u64).collect();

    // Initialize results buffer
    let mut results = vec![0u64; range_len];

    // Step 4: Create OpenCL buffers
    let buffer_numbers = Buffer::<u64>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_ONLY | flags::MEM_COPY_HOST_PTR)
        .len(range_len)
        .copy_host_slice(&numbers)
        .build()?;

    let buffer_results = Buffer::<u64>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_WRITE_ONLY)
        .len(range_len)
        .build()?;

    // Step 5: Set kernel arguments
    kernel.set_arg(0, &buffer_numbers)?;
    kernel.set_arg(1, &buffer_results)?;

    // Step 6: Execute the kernel with specified Global Work Size
    unsafe {
        kernel.cmd()
            .global_work_size([range_len as usize]) // Specify global work size
            .enq()?;
    }

    // Step 7: Read the results
    buffer_results.read(&mut results).enq()?;

    // Step 8: Collect prime numbers based on results with Progress Bar
    let pb = ProgressBar::new(range_len as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%, {eta_precise})")?
        .progress_chars("=>-"));
    pb.set_message("Collecting Primes");

    let mut primes = Vec::new();

    for (idx, &is_prime) in results.iter().enumerate() {
        if is_prime == 1 {
            primes.push(range[idx]);
        }
        pb.inc(1);
    }

    pb.finish_with_message("Prime Collection Completed");

    Ok(primes)
}

/// Writes the provided prime numbers to a file.
///
/// # Arguments
///
/// * `primes` - An iterator over prime numbers.
/// * `filename` - The name of the file to write the primes to.
pub fn write_primes_to_file(primes: &[u128], filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut writer = std::io::BufWriter::new(file);

    let pb = ProgressBar::new(primes.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%, {eta_precise})")?
        .progress_chars("=>-"));
    pb.set_message("Writing Primes to File");

    for &prime in primes {
        writeln!(writer, "{}", prime)?;
        pb.inc(1);
    }

    pb.finish_with_message("Prime Writing Completed");

    Ok(())
}