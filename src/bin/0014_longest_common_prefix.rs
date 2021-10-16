struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();
        let charsv: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

        for i in 0.. {
            let mut c: Option<char> = None;

            for s in &charsv {
                if i >= s.len() {
                    return ans;
                }

                match c {
                    None => c = Some(s[i]),
                    Some(ch) if s[i] != ch => return ans,
                    _ => continue,
                }
            }

            ans.push(charsv[0][i]);
        }

        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl".to_owned()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            "".to_owned()
        );
    }
}
