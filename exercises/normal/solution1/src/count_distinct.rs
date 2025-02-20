use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let result: HashSet<String> = input_str.split(",").map(|s| s.to_string()).collect();
    result.len()
}
