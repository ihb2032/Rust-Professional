use std::collections::HashMap;

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num, from_base) = match parse_num_str(num_str) {
        Ok(result) => result,
        Err(_) => return "".to_string(),
    };

    let from_result = match u32::from_str_radix(num, from_base) {
        Ok(result) => result,
        Err(_) => return "".to_string(),
    };

    let result = convert_decimal_to_base(from_result, to_base);

    result
}

fn parse_num_str(num_str: &str) -> Result<(&str, u32), String> {
    if !num_str.contains('(') || !num_str.contains(')') {
        return Err("".to_string());
    }

    let parts: Vec<&str> = num_str.split('(').collect();
    if parts.len() != 2 {
        return Err("".to_string());
    }

    let num = parts[0];
    let base_str = parts[1].trim_end_matches(')');

    match base_str.parse::<u32>() {
        Ok(base) => Ok((num, base)),
        Err(_) => Err("".to_string()),
    }
}

fn convert_decimal_to_base(mut num: u32, to_base: u32) -> String {
    if to_base < 2 || to_base > 16 {
        return "".to_string();
    }

    if num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let alphabet = "0123456789abcdef";

    while num > 0 {
        let remainder = num % to_base;
        result.push(alphabet.chars().nth(remainder as usize).unwrap());
        num /= to_base;
    }

    result.chars().rev().collect()
}
