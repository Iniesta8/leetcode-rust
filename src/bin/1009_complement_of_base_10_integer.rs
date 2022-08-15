struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut mask = 0;
        while mask < n {
            mask = mask << 1 | 1
        }

        n ^ mask
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_complement() {
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(7), 0);
        assert_eq!(Solution::bitwise_complement(10), 5);
        assert_eq!(Solution::bitwise_complement(0), 1);
    }
}
