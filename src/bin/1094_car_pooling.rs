use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut load_per_location: HashMap<i32, i32> = HashMap::new();

        for trip in trips {
            let (passengers, start, end) = (trip[0], trip[1], trip[2]);

            for location in start..end {
                *load_per_location.entry(location).or_insert(0) += passengers;
            }
        }

        load_per_location.values().all(|x| *x <= capacity)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_car_pooling() {
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4),
            false
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5),
            true
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3),
            true
        );
        assert_eq!(
            Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11),
            true
        );
    }
}
