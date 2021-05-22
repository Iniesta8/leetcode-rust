use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert('}', '{');
        pairs.insert(']', '[');

        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            if pairs.contains_key(&c) {
                if stack.is_empty() {
                    return false;
                }
                if stack.last().unwrap() != pairs.get(&c).unwrap() {
                    return false;
                }
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
        assert_eq!(Solution::is_valid(String::from("]")), false);
        assert_eq!(Solution::is_valid(String::from("[")), false);
    }
}
