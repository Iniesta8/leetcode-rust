use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1: HashMap<i32, i32> = HashMap::new();
        let mut map2: HashMap<i32, i32> = HashMap::new();
        let mut res = Vec::new();

        for x in nums1.iter() {
            *map1.entry(*x).or_insert(0) += 1;
        }
        for x in nums2.iter() {
            *map2.entry(*x).or_insert(0) += 1;
        }

        for (k, v) in map1.iter() {
            if map2.contains_key(k) {
                let m = map2[k].min(*v);
                for _ in 0..m {
                    res.push(*k);
                }
            }
        }
        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_intersect() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
