struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one = 1;
        let mut two = 1;

        for _ in 0..n - 1 {
            let tmp = one;
            one = one + two;
            two = tmp;
        }

        return one;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
