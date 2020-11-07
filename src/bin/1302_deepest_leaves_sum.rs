use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn count(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut sum: &mut i32,
        depth: i32,
        best_depth: &mut i32,
    ) {
        if let Some(node) = root {
            if depth > *best_depth {
                *sum = 0;
                *best_depth = depth;
            }

            if let Some(left) = node.borrow().left.clone() {
                Self::count(Some(left), &mut sum, depth + 1, best_depth);
            }
            if let Some(right) = node.borrow().right.clone() {
                Self::count(Some(right), &mut sum, depth + 1, best_depth);
            }

            if depth == *best_depth {
                *sum += node.borrow().val;
            }
        }
    }
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::count(root, &mut sum, 0, &mut 0);
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
    fn test_deepest_leaves_sum() {
        assert_eq!(
            Solution::deepest_leaves_sum(tree![
                1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8
            ]),
            15
        );
    }
}
