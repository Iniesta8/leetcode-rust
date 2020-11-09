struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut len = 0;
        let mut chars = vec![];

        for a in s.chars() {
            if let Some(idx) = chars.iter().position(|&x| x == a) {
                chars = chars[idx + 1..].to_vec();
            }
            chars.push(a);
            len = len.max(chars.len());
        }
        len as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
