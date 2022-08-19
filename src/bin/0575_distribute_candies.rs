use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn distribute_candies_v1(candy_type: Vec<i32>) -> i32 {
        let can_eat = candy_type.len() / 2;
        let seen_candies: HashSet<i32> = candy_type.into_iter().collect();

        std::cmp::min(seen_candies.len(), can_eat) as i32
    }

    pub fn distribute_candies_v2(mut candy_type: Vec<i32>) -> i32 {
        let can_eat = candy_type.len() / 2;

        let mut unique = 1;

        candy_type.sort();

        for i in 1..candy_type.len() {
            if candy_type[i] != candy_type[i - 1] {
                unique += 1;
            }
        }

        std::cmp::min(unique, can_eat) as i32
    }

    pub fn distribute_candies_v3(mut candy_type: Vec<i32>) -> i32 {
        let can_eat = candy_type.len() / 2;

        candy_type.sort();
        candy_type.dedup();

        std::cmp::min(candy_type.len(), can_eat) as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies_v3(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies_v3(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies_v3(vec![6, 6, 6, 6]), 1);
        assert_eq!(
            Solution::distribute_candies_v3(vec![
                100000, 0, 100000, 0, 100000, 0, 100000, 0, 100000, 0, 100000, 0
            ]),
            2
        );
    }
}
