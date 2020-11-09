struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let (mut red, mut white, mut blue) = (0, 0, nums.len() - 1);

        while white <= blue && blue > 0 {
            match nums[white] {
                0 => {
                    nums.swap(white, red);
                    red += 1;
                    white += 1;
                }
                1 => white += 1,
                _ => {
                    nums.swap(white, blue);
                    blue -= 1;
                }
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);

        nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);

        nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);

        nums = vec![2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2]);

        nums = vec![1, 2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
