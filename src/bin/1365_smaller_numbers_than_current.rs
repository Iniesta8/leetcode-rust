use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut cpy = nums.clone();
        cpy.sort();

        for (i, e) in cpy.iter().enumerate() {
            map.entry(*e).or_insert(i as i32);
        }

        nums.iter().map(|x| *map.get(&x).unwrap()).collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
