struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut start = n - 1;
        let mut res = 0;

        while start > 0 {
            res += start;
            start -= 2;
        }

        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(3), 2);
        assert_eq!(Solution::min_operations(6), 9);
    }
}
