struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|p| word.contains(&p.as_str()))
            .count() as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_of_strings() {
        use super::*;

        assert_eq!(
            Solution::num_of_strings(
                vec!["a".into(), "abc".into(), "bc".into(), "d".into()],
                "abc".into()
            ),
            3
        );
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".into(), "b".into(), "c".into()],
                "aaaaabbbbb".into()
            ),
            2
        );
    }
}
