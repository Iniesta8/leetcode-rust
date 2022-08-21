use leetcode_rust::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.unwrap();
        let val = node.borrow().val;

        match val {
            0 => false,
            1 => true,
            2 => {
                Solution::evaluate_tree(node.borrow().left.clone())
                    || Solution::evaluate_tree(node.borrow().right.clone())
            }
            3 => {
                Solution::evaluate_tree(node.borrow().left.clone())
                    && Solution::evaluate_tree(node.borrow().right.clone())
            }
            _ => panic!("Unknown node property"),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_evaluate_tree() {
        assert_eq!(
            Solution::evaluate_tree(tree![2, 1, 3, null, null, 0, 1]),
            true
        );
        assert_eq!(Solution::evaluate_tree(tree![0]), false);
    }
}
