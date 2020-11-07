use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode_rust::util::tree::TreeNode;

struct Solution;

impl Solution {
    fn collect_values(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut values: &mut HashMap<i32, i32>,
        mut max_count: &mut i32,
    ) {
        if let Some(node) = root {
            Self::collect_values(node.borrow().left.clone(), &mut values, &mut max_count);
            Self::collect_values(node.borrow().right.clone(), &mut values, &mut max_count);

            *values.entry(node.borrow().val).or_insert(0) += 1;
            *max_count = *max_count.max(values.get_mut(&node.borrow().val).unwrap());
        }
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;

        Self::collect_values(root, &mut values, &mut max_count);

        if values.is_empty() {
            return vec![];
        }

        values
            .into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(val, _)| val)
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::tree;
    use leetcode_rust::util::tree::to_tree;

    #[test]
    fn test_find_mode() {
        assert_eq!(Solution::find_mode(tree![1, null, 2, 2]), vec![2]);
    }
}
