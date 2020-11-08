struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut dists: Vec<i32> = points.iter().map(|cords| cords[0]).collect();
        dists.sort_unstable();
        dists.windows(2).map(|w| w[1] - w[0]).max().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_width_of_vertical_area() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1
        );
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8],
            ]),
            3
        );
    }
}
