pub fn find_max_prime_factor(mut number: u128) -> u128 {
    if number == 0 {
        return 0;
    }
    let mut max_prime = 1;

    for &p in [2, 3, 5].iter() {
        if number % p == 0 {
            max_prime = p;
            while number % p == 0 {
                number /= p;
            }
        }
        if number == 1 {
            return max_prime;
        }
    }

    let remaining_factors = factorize(number);
    let current_max = remaining_factors.into_iter().max().unwrap_or(1);
    max_prime.max(current_max)
}


fn factorize(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    if n == 1 {
        return factors;
    }
    if is_prime(n) {
        factors.push(n);
        return factors;
    }
    let d = pollards_rho(n);
    factors.extend(factorize(d));
    factors.extend(factorize(n / d));
    factors
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut cont = false;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                cont = true;
                break;
            }
        }
        if cont {
            continue;
        }
        return false;
    }
    true
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp /= 2;
    }
    result
}

fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b % m;
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    result
}

fn add_mod(a: u128, b: u128, modulus: u128) -> u128 {
    (a + b) % modulus
}

fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;
    let mut c = 1;

    let f = |x: u128, c: u128| -> u128 {
        add_mod(mul_mod(x, x, n), c, n)
    };

    while d == 1 {
        x = f(x, c);
        y = f(f(y, c), c);
        d = gcd(x.abs_diff(y), n);
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}