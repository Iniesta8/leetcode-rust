struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        // oneliner:
        // (x ^ y).count_ones() as i32

        let mut sum = 0;

        let mut xi = x;
        let mut yi = y;

        while xi != 0 || yi != 0 {
            sum += (xi % 2) ^ (yi % 2);
            xi /= 2;
            yi /= 2;
        }

        sum
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
