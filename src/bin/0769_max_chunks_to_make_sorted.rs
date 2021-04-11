struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut chunks = 0;
        let mut max_value = -1;

        for (i, e) in arr.iter().enumerate() {
            max_value = std::cmp::max(max_value, *e);
            if max_value == i as i32 {
                chunks += 1;
            }
        }
        chunks
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_chunks_to_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
