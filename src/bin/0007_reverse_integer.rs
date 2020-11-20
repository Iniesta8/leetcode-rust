struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum()
            * x.abs()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
