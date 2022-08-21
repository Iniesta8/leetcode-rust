struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut ans = vec![0, 1, 1]; // constant space!

        for i in 3..=n {
            ans[(i % 3) as usize] = ans[0] + ans[1] + ans[2];
        }

        ans[(n % 3) as usize]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
        assert_eq!(Solution::tribonacci(35), 615693474);
    }
}
