use std::cmp;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_letters = HashMap::new();
        let mut ans = vec![];
        let mut left = 0;
        let mut right = 0;

        for (i, c) in s.chars().enumerate() {
            last_letters.insert(c, i);
        }

        for (i, c) in s.chars().enumerate() {
            right = cmp::max(right, *last_letters.get(&c).unwrap());
            if i == right {
                ans.push(i as i32 - left + 1);
                left = i as i32 + 1;
            }
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }
}
