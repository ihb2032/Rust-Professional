/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    let mut start = 0;
    let mut end = 0;
    let mut max_len = 0;
    let mut char_index_map: HashMap<char, usize> = HashMap::new();
    while end < s.len() {
        let char = s.chars().nth(end).unwrap();
        if let Some(&index) = char_index_map.get(&char) {
            if index >= start {
                start = index + 1;
            }
        }
        char_index_map.insert(char, end);
        max_len = max(max_len, (end - start + 1) as i32);
        end += 1;
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}
