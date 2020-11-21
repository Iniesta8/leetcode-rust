struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = None;
        let mut second = None;
        let mut third = None;
        let mut helper;

        for val in nums.into_iter() {
            let val = Some(val);
            if val > first {
                helper = second;
                second = first;
                first = val;
                third = helper;
            } else if val < first && val > second {
                helper = second;
                second = val;
                third = helper;
            } else if val < second && val >= third {
                third = val;
            }
        }

        third.unwrap_or(first.unwrap())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
