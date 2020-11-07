use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn is_mirror(r1: Option<&Rc<RefCell<TreeNode>>>, r2: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (r1, r2) {
            (Some(r1), Some(r2)) => {
                let (r1, r2) = (r1.borrow(), r2.borrow());
                r1.val == r2.val
                    && Self::is_mirror(r1.right.as_ref(), r2.left.as_ref())
                    && Self::is_mirror(r1.left.as_ref(), r2.right.as_ref())
            }
            (None, None) => true,
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(root.as_ref(), root.as_ref())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_is_symmetric() {
        assert_eq!(Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]), true);
        assert_eq!(
            Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]),
            false
        );
        assert_eq!(Solution::is_symmetric(tree![]), true);
    }
}
