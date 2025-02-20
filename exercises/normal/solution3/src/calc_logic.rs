pub fn new_birthday_probability(n: u32) -> f64 {
    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }

    let probability_same = 1.0 - probability;

    (probability_same * 10000.0).round() / 10000.0
}