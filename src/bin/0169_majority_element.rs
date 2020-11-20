struct Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
