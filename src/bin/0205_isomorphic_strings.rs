use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut mappings: HashMap<char, char> = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            if let Some(c) = mappings.get(&a) {
                if *c != b {
                    return false;
                }
            } else if mappings.values().any(|c| *c == b) {
                return false;
            } else {
                mappings.insert(a, b);
            }
        }

        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic(String::from("egg"), String::from("add")),
            true
        );
        assert_eq!(
            Solution::is_isomorphic(String::from("foo"), String::from("bar")),
            false
        );
        assert_eq!(
            Solution::is_isomorphic(String::from("paper"), String::from("title")),
            true
        );
        assert_eq!(
            Solution::is_isomorphic(String::from("badc"), String::from("baba")),
            false
        );
    }
}
