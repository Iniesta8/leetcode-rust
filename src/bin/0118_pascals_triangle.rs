struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = (1..=num_rows)
            .into_iter()
            .map(|n| vec![1; n as usize])
            .collect();

        for row in 1..=num_rows {
            for j in 1..row - 1 {
                triangle[(row - 1) as usize][j as usize] = triangle[(row - 2) as usize]
                    [(j - 1) as usize]
                    + triangle[(row - 2) as usize][j as usize];
            }
        }

        triangle
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
