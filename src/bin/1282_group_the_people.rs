struct Group {
    persons: Vec<i32>,
    cur_size: i32,
    max_size: i32,
}
struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups: Vec<Group> = Vec::new();

        'outer: for (i, gsize) in group_sizes.iter().enumerate() {
            for g in &mut groups {
                if g.max_size == *gsize && g.cur_size < *gsize {
                    g.persons.push(i as i32);
                    g.cur_size += 1;
                    continue 'outer;
                }
            }

            groups.push(Group {
                max_size: *gsize,
                persons: vec![i as i32],
                cur_size: 1,
            });
        }
        groups.iter().map(|x| x.persons.clone()).collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_the_people() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![0, 1, 2], vec![3, 4, 6], vec![5]]
        );
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec![vec![0, 5], vec![1], vec![2, 3, 4]]
        );
    }
}
