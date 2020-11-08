use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if let Some(node) = root {
            let mut sum = 0;
            let nv = node.borrow().val;

            if l <= nv && nv <= r {
                sum += nv;
            }
            if l < nv {
                sum += Self::range_sum_bst(node.borrow().left.clone(), l, r);
            }
            if nv < r {
                sum += Self::range_sum_bst(node.borrow().right.clone(), l, r);
            }
            return sum;
        }
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_range_sum_bst() {
        assert_eq!(
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, null, 18], 7, 15),
            32
        );
        assert_eq!(
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6], 6, 10),
            23
        );
    }
}
