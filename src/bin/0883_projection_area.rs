struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let xy = grid.iter().flatten().filter(|&&x| x != 0).count() as i32;

        let xz: i32 = grid.iter().map(|r| r.iter().max().unwrap()).sum();

        let mut yz = 0;
        let mut v = vec![0; grid[0].len()];
        for i in 0..v.len() {
            for r in grid.iter() {
                v[i] = std::cmp::max(v[i], r[i]);
            }
            yz += v[i];
        }

        xy + xz + yz
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
        assert_eq!(
            Solution::projection_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            14
        );
    }
}
