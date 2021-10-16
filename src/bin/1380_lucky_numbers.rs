struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut mins = vec![i32::MAX; rows];
        let mut maxs = vec![i32::MIN; cols];

        for (idx, row) in matrix.iter().enumerate() {
            mins[idx] = *row.iter().min().unwrap();
        }

        for col in 0..matrix[0].len() {
            maxs[col] = matrix.iter().map(|row| row[col]).max().unwrap();
        }

        for min in &mins {
            for max in &maxs {
                if max == min {
                    return vec![*max];
                }
            }
        }

        vec![]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucky_numbers() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
