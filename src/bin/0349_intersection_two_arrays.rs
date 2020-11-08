use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.iter().cloned().collect();
        let set2: HashSet<i32> = nums2.iter().cloned().collect();

        set1.intersection(&set2).copied().collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersection() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
