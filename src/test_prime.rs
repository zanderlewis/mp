use num_bigint::BigUint;
use num_traits::{One, Zero};
use rayon::prelude::*;
use num_integer::Integer;
use std::sync::Arc;

pub fn lucas_lehmer(p: u128) -> bool {
    if p == 2 {
        return true;
    }

    let m = (&BigUint::one() << p) - 1u32;
    let iterations = p - 2;
    let s = Arc::new(std::sync::Mutex::new(BigUint::from(4u32)));

    (0..iterations).into_par_iter().for_each(|_| {
        let mut s_lock = s.lock().unwrap();
        *s_lock = (&*s_lock * &*s_lock - 2u32) % &m;
    });

    let final_s = s.lock().unwrap();
    final_s.is_zero()
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
