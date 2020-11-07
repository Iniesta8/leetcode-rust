struct Solution;

// Constraints:
//
// n == nums.length
// m == l.length
// m == r.length
// 2 <= n <= 500
// 1 <= m <= 500
// 0 <= l[i] < r[i] < n
// -105 <= nums[i] <= 105

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        assert_eq!(l.len(), r.len());

        l.iter()
            .zip(r.iter())
            .map(|(&li, &ri)| {
                if ri - li == 1 {
                    return true;
                }

                // there are at least 3 elements now
                let mut nums_tmp: Vec<i32> =
                    nums[li as usize..=ri as usize].iter().copied().collect();

                nums_tmp.sort();

                let dist = nums_tmp[1] - nums_tmp[0];
                nums_tmp.windows(2).skip(1).all(|c| (c[1] - c[0]) == dist)
            })
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_arithmetic_subarrays1() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
    }

    #[test]
    fn test_check_arithmetic_subarrays2() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10],
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
