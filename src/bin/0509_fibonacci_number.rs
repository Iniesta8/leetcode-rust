struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut dp = [0, 1];
        for i in 1..=n {
            dp[(i % 2) as usize] = dp[0] + dp[1];
        }
        dp[(n % 2) as usize]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
