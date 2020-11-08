use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn count(root: Option<Rc<RefCell<TreeNode>>>, mut sum: &mut i32, is_left_leaf: &mut bool) {
        if let Some(node) = root {
            Self::count(node.borrow().left.clone(), &mut sum, &mut true);
            Self::count(node.borrow().right.clone(), &mut sum, &mut false);
            if node.borrow().left.is_none() && node.borrow().right.is_none() && *is_left_leaf {
                *sum += node.borrow().val;
            }
        }
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::count(root, &mut sum, &mut false);
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
    fn test_sum_of_left_leaves() {
        assert_eq!(
            Solution::sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7]),
            24
        );
    }
}
