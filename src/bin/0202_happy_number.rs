use std::collections::HashSet;

struct Solution;

impl Solution {
    fn squared_digits_sum(mut n: i32) -> i32 {
        let mut sum = 0;

        while n > 0 {
            let val = n % 10;
            sum += val * val;
            n = n / 10;
        }
        sum
    }

    fn squared_digits_sum_func(n: i32) -> i32 {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .map(|d| d * d)
            .sum()
    }

    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut res = n;

        while res != 1 {
            res = Solution::squared_digits_sum_func(res);
            if !seen.insert(res) {
                return false;
            }
        }

        true
    }
}

fn main() {
    Solution::is_happy(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
        assert_eq!(Solution::is_happy(3), false);
    }
}
