struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let xs = x.abs().to_string();
        let half = xs.len() / 2;

        xs.chars()
            .take(half)
            .zip(xs.chars().rev().take(half))
            .all(|(x, y)| x == y)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(100), false);
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
