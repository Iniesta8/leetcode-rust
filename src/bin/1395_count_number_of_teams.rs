struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = rating.len();

        // for each soldier...
        for i in 0..len {
            let (mut ll, mut lg, mut rl, mut rg) = (0, 0, 0, 0);
            // ...count all soldiers on the left that rating is less/greater
            for j in 0..i {
                if rating[j] < rating[i] {
                    ll += 1;
                } else if rating[j] > rating[i] {
                    lg += 1;
                }
            }
            // ...count all soldiers on the right that rating is less/greater
            for j in i + 1..len {
                if rating[j] < rating[i] {
                    rl += 1;
                } else if rating[j] > rating[i] {
                    rg += 1;
                }
            }
            count += ll * rg + lg * rl;
        }
        count
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_teams() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
