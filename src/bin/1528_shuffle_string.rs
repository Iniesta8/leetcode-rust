struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ans = vec![' '; s.len()];

        indices.into_iter().zip(s.chars()).for_each(|(i, c)| {
            ans[i as usize] = c;
        });

        ans.iter().collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_string() {
        assert_eq!(
            Solution::restore_string(String::from("codeleet"), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            String::from("leetcode")
        );
    }
}
