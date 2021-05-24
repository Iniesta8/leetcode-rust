struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|l, r| {
            if l[0] == r[0] {
                r[1].cmp(&l[1])
            } else {
                l[0].cmp(&r[0])
            }
        });

        let mut ub = -1;
        let mut ans = 0;

        for i in 0..intervals.len() {
            if intervals[i][1] > ub {
                ub = intervals[i][1];
                ans += 1;
            }
        }

        ans

        // Alternative solution: actually removing covered intervals from list
        // let mut i = 1;
        // while i < intervals.len() {
        //     let (lhs, rhs) = (&intervals[i - 1], &intervals[i]);
        //
        //     if lhs[0] <= rhs[0] && lhs[1] >= rhs[1] {
        //         intervals.remove(i);
        //     } else if rhs[0] <= lhs[0] && rhs[1] >= lhs[1] {
        //         intervals.remove(i - 1);
        //     } else {
        //         i += 1;
        //     }
        // }
        //
        // intervals.len() as i32
    }
}

fn main() {
    let ans = Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]);
    println!("ans = {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
            1
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![0, 10], vec![5, 12]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]]),
            2
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]]),
            1
        );
        assert_eq!(
            Solution::remove_covered_intervals(vec![
                vec![66672, 75156],
                vec![59890, 65654],
                vec![92950, 95965],
                vec![9103, 31953],
                vec![54869, 69855],
                vec![33272, 92693],
                vec![52631, 65356],
                vec![43332, 89722],
                vec![4218, 57729],
                vec![20993, 92876]
            ]),
            3
        );
    }
}
