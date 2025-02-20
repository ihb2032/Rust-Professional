pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_year: i32 = time[..4].parse().unwrap();
    let birth_month: i32 = time[5..].parse().unwrap();

    let (base_age, months_per_delay, target_age) = match tp {
        "男职工" => (60, 4, 63),
        "原法定退休年龄55周岁女职工" => (55, 4, 58),
        "原法定退休年龄50周岁女职工" => (50, 2, 55),
        _ => panic!("未知人员类型"),
    };

    let mut retire_year = birth_year + base_age;
    let mut retire_month = birth_month;
    let mut delay_months = 0;
    let policy_start = 2025 * 12 + 1;
    let retire_total_months = retire_year * 12 + retire_month;

    if retire_total_months >= policy_start {
        let months_since_policy = retire_total_months - policy_start;
        delay_months = ((months_since_policy + months_per_delay) / months_per_delay)
            .min((target_age - base_age) * 12);
        let new_total_months = retire_total_months + delay_months;
        retire_year = new_total_months / 12;
        retire_month = new_total_months % 12;
        if retire_month == 0 {
            retire_month = 12;
            retire_year -= 1;
        }
    }

    let retire_age = (retire_year - birth_year) as f64 + (retire_month - birth_month) as f64 / 12.0;
    let retire_age_str = if (retire_age * 100.0).round() % 100.0 == 0.0 {
        format!("{}", retire_age as i32)
    } else {
        format!("{:.2}", retire_age)
    };

    format!(
        "{:04}-{:02},{},{}",
        retire_year, retire_month, retire_age_str, delay_months
    )
}
