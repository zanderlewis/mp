use num_bigint::BigUint;
use num_traits::One;
use num_integer::Integer;
use ocl::{flags, ProQue};
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;

pub fn lucas_lehmer(p: u128, mem: bool) -> Result<bool, Box<dyn Error>> {
    if p == 2 {
        return Ok(true);
    }

    // Construct Mersenne number M = 2^p - 1
    let m = (&BigUint::one() << p) - 1u32;
    let iterations = p - 2;

    // OpenCL kernel source code
    let src = r#"
    __kernel void lucas_lehmer(__global ulong* s, __global const ulong* m) {
        ulong a = s[0];
        // Perform s = (s * s - 2) mod m
        ulong result = (a * a - 2) % m[0];

        if (a == 0) {
            s[0] = 0;
        } else {
            s[0] = result;
        }
    }
    "#;

    // Initialize OpenCL
    let pro_que = ProQue::builder()
        .src(src)
        .dims(1)
        .build()?;

    // Ensure M fits in u64
    let m_u64 = match m.to_u64_digits().get(0) {
        Some(&num) => num,
        None => {
            return Err("Mersenne number exceeds u64 limit.".into());
        }
    };
    let mut s_host = vec![4u64];
    let m_host = vec![m_u64];

    // Create buffers using buffer_builder from ProQue
    let s_buffer = pro_que.buffer_builder()
        .flags(flags::MEM_READ_WRITE)
        .len(1)
        .copy_host_slice(&s_host)
        .build()?;

    let m_buffer = pro_que.buffer_builder()
        .flags(flags::MEM_READ_ONLY)
        .len(1)
        .copy_host_slice(&m_host)
        .build()?;

    // Build the kernel and set arguments
    let kernel = pro_que.kernel_builder("lucas_lehmer")
        .arg(&s_buffer)
        .arg(&m_buffer)
        .build()?;

    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // Initialize the progress bar
    let pb = ProgressBar::new(iterations as u64);
    let style = ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta_precise})")?
        .progress_chars("=>-");
    pb.set_style(style);
    pb.set_message("Performing Lucas-Lehmer Test");

    let mut current_iteration = 0u128;
    let state_file = "lucas_lehmer_state.bin";

    if mem {
        // Initialize or load state
        if Path::new(state_file).exists() {
            // Load saved state
            let mut file = File::open(state_file)?;
            let mut buffer = [0u8; 8];
            file.read_exact(&mut buffer)?;
            s_host[0] = u64::from_le_bytes(buffer);
            let mut buffer_iter = [0u8; 16];
            file.read_exact(&mut buffer_iter)?;
            current_iteration = u128::from_le_bytes(buffer_iter);
            // Update buffer
            s_buffer.write(&s_host).enq()?;
            println!("Resuming from iteration {}", current_iteration);
        }
    }

    pb.set_position(current_iteration as u64);

    if mem {
        for i in current_iteration..iterations {
            unsafe {
                kernel.enq()?;
            }
            pb.inc(1);
            
            // Every 100,000,000 iterations, save state
            if (i + 1) % 100_000_000 == 0 {
                // Read the current s_host
                s_buffer.read(&mut s_host).enq()?;

                // Save s_host[0] and current_iteration to file
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(state_file)?;
                file.write_all(&s_host[0].to_le_bytes())?;
                file.write_all(&(i + 1).to_le_bytes())?;
                file.flush()?;
            }
        }
    } else {
        for _ in current_iteration..iterations {
            unsafe {
                kernel.enq()?;
            }
            pb.inc(1);
        }
    }

    // Finish the progress bar
    pb.finish_with_message("Lucas-Lehmer Test Completed");

    // Read the result back to host
    s_buffer.read(&mut s_host).enq()?;

    if mem {
        // Remove saved state file
        if Path::new(state_file).exists() {
            std::fs::remove_file(state_file)?;
        }
    }

    // Print the result
    let message = format!("{} is {}a Mersenne prime.", m, if s_host[0] == 0 { "" } else { "not " });
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.txt")?;
    file.write_all(message.as_bytes())?;
    file.flush()?;

    Ok(s_host[0] == 0)
}

pub fn is_prp(n: &BigUint, base: u128) -> bool {
    let mut d = n - 1u32;
    let mut s = 0;

    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    let mut x = BigUint::from(base).modpow(&d, n);
    if x.is_one() || x == n - 1u32 {
        return true;
    }

    for _ in 0..s - 1 {
        x = (&x * &x) % n;
        if x.is_one() {
            return false;
        }
        if x == n - 1u32 {
            return true;
        }
    }

    false
}
