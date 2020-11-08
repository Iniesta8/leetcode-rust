struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        let mut horizontal: [i32; 50] = [0; 50];
        let mut vertical: [i32; 50] = [0; 50];

        for c in 0..grid[1].len() {
            for l in 0..grid[0].len() {
                vertical[c] = vertical[c].max(grid[l][c]);
                horizontal[l] = horizontal[l].max(grid[l][c]);
            }
        }

        for l in 0..grid[0].len() {
            for c in 0..grid[1].len() {
                count += horizontal[l].min(vertical[c]) - grid[l][c];
            }
        }

        count
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_increase_keeping_skyline() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0],
            ]),
            35
        );
    }
}
