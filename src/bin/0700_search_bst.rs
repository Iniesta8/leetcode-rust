use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let nv = node.borrow().val;

            if val < nv {
                return Self::search_bst(node.borrow().left.clone(), val);
            } else if val > nv {
                return Self::search_bst(node.borrow().right.clone(), val);
            }
        }
        root
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_search_bst() {
        assert_eq!(
            Solution::search_bst(tree![4, 2, 7, 1, 3, null, null], 2),
            tree![2, 1, 3]
        );
    }
}
