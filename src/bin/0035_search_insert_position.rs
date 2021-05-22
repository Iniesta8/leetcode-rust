struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut m = i32::MIN;
        for (i, &n) in nums.iter().enumerate() {
            if n == target || (m < target && target < n) {
                return i as i32;
            }
            m = n;
        }
        nums.len() as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }
}
