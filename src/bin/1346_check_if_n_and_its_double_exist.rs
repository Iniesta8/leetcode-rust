use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();

        for val in arr {
            dbg!(val);
            if val % 2 == 0 && seen.contains(&(val / 2)) {
                return true;
            }
            if seen.contains(&(val * 2)) {
                return true;
            }
            seen.insert(val);
        }

        false
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_exist() {
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
        assert_eq!(Solution::check_if_exist(vec![7, 1, 14, 11]), true);
        assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
        assert_eq!(
            Solution::check_if_exist(vec![-20, 8, -6, -14, 0, -19, 14, 4]),
            true
        );
    }
}
