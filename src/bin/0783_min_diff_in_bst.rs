use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            // the following order is import to get the values of the BST sorted
            Self::traverse(node.borrow().left.clone(), vals);
            vals.push(node.borrow().val);
            Self::traverse(node.borrow().right.clone(), vals);
        }
    }
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals = vec![];
        Self::traverse(root, &mut vals);

        vals.windows(2).map(|c| c[1] - c[0]).min().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_min_diff_in_bst() {
        assert_eq!(
            Solution::min_diff_in_bst(tree![4, 2, 6, 1, 3, null, null]),
            1
        );
    }
}
