use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, smallest: i32, mut val: &mut i32) {
        if let Some(node) = root {
            let n = node.borrow().val;

            if n > smallest {
                if *val == smallest {
                    *val = n;
                } else {
                    *val = n.min(*val);
                    return;
                }
            }
            Self::traverse(node.borrow().left.clone(), smallest, &mut val);
            Self::traverse(node.borrow().right.clone(), smallest, &mut val);
        }
    }

    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let smallest = root.clone().unwrap().borrow().val;
        let mut val = smallest;
        Self::traverse(root, smallest, &mut val);

        if smallest == val {
            return -1;
        }
        val
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_find_second_minimum_value() {
        assert_eq!(
            Solution::find_second_minimum_value(tree![2, 2, 25, null, null, 5, 7]),
            5
        );
        assert_eq!(Solution::find_second_minimum_value(tree![2, 2, 2]), -1);
        assert_eq!(Solution::find_second_minimum_value(tree![2, 2, 2]), -1);
    }
}
