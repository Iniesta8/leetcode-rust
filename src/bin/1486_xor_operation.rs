struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n)
            .into_iter()
            .map(|e| start + 2 * e)
            .fold(0, |acc, x| acc ^ x)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xor_operation() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
        assert_eq!(Solution::xor_operation(1, 7), 7);
        assert_eq!(Solution::xor_operation(10, 5), 2);
    }
}
