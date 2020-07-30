pub fn nth(n: u32) -> u32 {
    let mut found_primes = 0;
    let mut i = 2;
    loop {
        if is_prime(i) {
            if found_primes == n {
                return i;
            }
            found_primes += 1;
        }
        i += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n > 7 && (n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0) {
        return false;
    }

    let f: f64 = n as f64;
    for i in 2..=f.sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
