struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}

fn main() {
    let res = Solution::num_water_bottles(32, 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_bottles() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
        assert_eq!(Solution::num_water_bottles(5, 5), 6);
        assert_eq!(Solution::num_water_bottles(2, 3), 2);
    }
}
