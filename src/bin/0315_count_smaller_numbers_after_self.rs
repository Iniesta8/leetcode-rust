struct Solution;

impl Solution {
    // Not the fastest, but the shortest solution
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .map(|(i, e)| nums[i + 1..].iter().filter(|&x| x < e).count() as i32)
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_smaller() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }
}
