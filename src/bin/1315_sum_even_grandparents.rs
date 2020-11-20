use leetcode_rust::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        parent: &mut Option<i32>,
        grand_parent: &mut Option<i32>,
        sum: &mut i32,
    ) {
        if let Some(node) = root {
            if let Some(gp) = grand_parent {
                if *gp % 2 == 0 {
                    *sum += node.borrow().val;
                }
            }

            Self::traverse(
                node.borrow().left.clone(),
                &mut Some(node.borrow().val),
                parent,
                sum,
            );
            Self::traverse(
                node.borrow().right.clone(),
                &mut Some(node.borrow().val),
                parent,
                sum,
            );
        }
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        Self::traverse(root, &mut None, &mut None, &mut sum);

        sum
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_sum_even_grandparent() {
        assert_eq!(
            Solution::sum_even_grandparent(tree![
                6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
            ]),
            18
        );
    }
}
