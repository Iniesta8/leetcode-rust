use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: &mut i32, max_depth: &mut i32) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                *max_depth = *max_depth.max(depth);
            }
            Self::dfs(node.borrow().left.clone(), &mut (*depth + 1), max_depth);
            Self::dfs(node.borrow().right.clone(), &mut (*depth + 1), max_depth);
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max_depth = std::i32::MIN;

        Self::dfs(root, &mut 1, &mut max_depth);

        max_depth
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
    }
}
