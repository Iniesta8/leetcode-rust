struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut mx = 0;
        let mut counter = 0;

        for c in s.chars() {
            if c == '(' {
                counter += 1;
                mx = std::cmp::max(mx, counter);
            } else if c == ')' {
                counter -= 1;
            }
        }
        mx
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth(String::from("(1+(2*3)+((8)/4))+1")), 3);
        assert_eq!(Solution::max_depth(String::from("(1)+((2))+(((3)))")), 3);
        assert_eq!(Solution::max_depth(String::from("1+(2*3)/(2-1)")), 1);
        assert_eq!(Solution::max_depth(String::from("1")), 0);
    }
}
