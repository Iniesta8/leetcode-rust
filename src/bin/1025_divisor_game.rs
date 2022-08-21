struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        // n = 1 => Alice loses
        // n = 2 => Alice can win if she chooses x = 1
        // n = 3 => Alice loses
        // n = 4 => Alice can win if she chooses x = 1
        // n = 5 => Alice loses
        // n = 6 => Alice can win if she chooses x = 1
        // ...
        //
        // --> Alice can win if n is even, if n is odd Bob wins
        // --> Alice has to take x = 1 in any case (?)

        n % 2 == 0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_game() {
        assert_eq!(Solution::divisor_game(2), true);
        assert_eq!(Solution::divisor_game(3), false);
    }
}
