struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&x| x != 0);

        for _ in 0..len - nums.len() {
            nums.push(0);
        }
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }
}
