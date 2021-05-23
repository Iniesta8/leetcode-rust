use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = vec![];
        if let Some(node) = root {
            values.append(&mut Self::inorder_traversal(node.borrow().left.clone()));
            values.push(node.borrow().val);
            values.append(&mut Self::inorder_traversal(node.borrow().right.clone()));
        }
        values
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::inorder_traversal(tree![1, null, 2, 3]), [1, 3, 2]);
        assert_eq!(Solution::inorder_traversal(tree![]), []);
        assert_eq!(Solution::inorder_traversal(tree![1]), [1]);
        assert_eq!(Solution::inorder_traversal(tree![1, 2]), [2, 1]);
        assert_eq!(Solution::inorder_traversal(tree![1, null, 2]), [1, 2]);
    }
}
