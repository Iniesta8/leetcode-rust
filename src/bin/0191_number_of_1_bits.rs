struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut sum = 0;
        let mut i = n;

        while i != 0 {
            sum += i % 2;
            i /= 2;
        }

        sum as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(
            Solution::hamming_weight(
                u32::from_str_radix("00000000000000000000000000001011", 2).unwrap()
            ),
            3
        );
        assert_eq!(
            Solution::hamming_weight(
                u32::from_str_radix("00000000000000000000000010000000", 2).unwrap()
            ),
            1
        );
    }
}
