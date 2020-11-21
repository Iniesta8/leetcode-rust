use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: &mut i32, min_depth: &mut i32) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                *min_depth = *min_depth.min(depth);
            }
            Self::dfs(node.borrow().left.clone(), &mut (*depth + 1), min_depth);
            Self::dfs(node.borrow().right.clone(), &mut (*depth + 1), min_depth);
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut min_depth = std::i32::MAX;

        Self::dfs(root, &mut 1, &mut min_depth);

        min_depth
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_min_depth() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(
            Solution::min_depth(tree![2, null, 3, null, 4, null, 5, null, 6]),
            5
        );
    }
}
