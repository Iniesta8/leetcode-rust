use std::cell::RefCell;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

// Constraint: all node values are unique
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Self::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Self::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node;
                }
                r
            }
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_insert_into_bst() {
        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3], 5),
            tree![4, 2, 7, 1, 3, 5]
        );
        assert_eq!(
            Solution::insert_into_bst(tree![40, 20, 60, 10, 30, 50, 70], 25),
            tree![40, 20, 60, 10, 30, 50, 70, null, null, 25]
        );
        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3, null, null, null, null, null, null], 5),
            tree![4, 2, 7, 1, 3, 5]
        );
    }
}
