use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        if a.len() != b.len() {
            return false;
        }

        let mut diff = 0;
        let mut diff_vals: Option<(char, char)> = None;

        for (x, y) in a.chars().zip(b.chars()) {
            if x != y {
                diff += 1;
                if diff > 2 {
                    return false;
                }

                if let Some((xi, yi)) = diff_vals {
                    if xi != y || yi != x {
                        return false;
                    }
                } else {
                    diff_vals = Some((x, y));
                }
            } else {
                *map.entry(x).or_insert(0) += 1;
            }
        }

        diff == 2 || (diff == 0 && map.values().any(|x| *x >= 2))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_strings() {
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ba")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ab")),
            false
        );
        assert_eq!(
            Solution::buddy_strings(String::from("aa"), String::from("aa")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from(""), String::from("aa")),
            false
        );
        assert_eq!(
            Solution::buddy_strings(String::from("abcaa"), String::from("abcbb")),
            false
        );
        assert_eq!(
            Solution::buddy_strings(String::from("abac"), String::from("abad")),
            false
        );
    }
}
