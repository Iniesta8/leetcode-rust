struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut max_satisfied = 0;

        let certainly_satisfied: i32 = customers
            .iter()
            .zip(grumpy.iter())
            .filter(|(_, &g)| g == 0)
            .map(|(&c, _)| c)
            .sum();

        let mut l = 0;
        let mut r = minutes - 1;

        while r < customers.len() as i32 {
            let also_satisfied: i32 = (l..=r as usize)
                .into_iter()
                .map(|i| grumpy[i] * customers[i])
                .sum();

            max_satisfied = max_satisfied.max(certainly_satisfied + also_satisfied);

            l += 1;
            r += 1;
        }

        max_satisfied
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
        assert_eq!(
            Solution::max_satisfied(vec![10, 1, 7], vec![0, 0, 0], 2),
            18
        );
    }
}
