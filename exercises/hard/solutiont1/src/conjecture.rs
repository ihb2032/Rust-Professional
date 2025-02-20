pub fn goldbach_conjecture() -> String {
    let mut non_goldbach_numbers = vec![];
    for n in (9..).step_by(2) {
        if is_prime(n) {
            continue;
        }
        if !can_be_written_as_goldbach(n) {
            non_goldbach_numbers.push(n);
            if non_goldbach_numbers.len() == 2 {
                break;
            }
        }
    }
    non_goldbach_numbers
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
fn can_be_written_as_goldbach(n: usize) -> bool {
    for prime in 2..n {
        if is_prime(prime) {
            let remainder = n - prime;
            if is_twice_square(remainder) {
                return true;
            }
        }
    }
    false
}
fn is_twice_square(n: usize) -> bool {
    if n % 2 != 0 {
        return false;
    }
    let half = n / 2;
    let sqrt = (half as f64).sqrt() as usize;
    sqrt * sqrt == half
}
