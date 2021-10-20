struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(str::to_owned)
            .rev()
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned(),
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_owned()),
            "world hello".to_owned(),
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_owned()),
            "example good a".to_owned(),
        );
        assert_eq!(
            Solution::reverse_words("  Bob    Loves  Alice   ".to_owned()),
            "Alice Loves Bob".to_owned()
        );
    }
}
