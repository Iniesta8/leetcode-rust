use leetcode_rust::util::tree::TreeNode;
use std::cell::RefCell;
// use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    // fn traverse(
    // root: Option<Rc<RefCell<TreeNode>>>,
    // depth: i32,
    // mut values: &mut HashMap<i32, i32>,
    // ) {
    // if let Some(node) = root {
    // let mut val = node.borrow().val;
    // let cur = values.entry(depth).or_insert(val);
    // *cur = *cur.max(&mut val);
    //
    // Self::traverse(node.borrow().left.clone(), depth + 1, &mut values);
    // Self::traverse(node.borrow().right.clone(), depth + 1, &mut values);
    // }
    // }
    // pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // let mut values = HashMap::new();
    //
    // Self::traverse(root, 0, &mut values);
    //
    // (0..values.len())
    // .map(|i| *values.get(&(i as i32)).unwrap())
    // .collect()
    // }
    //
    //
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, mut values: &mut Vec<i32>) {
        if let Some(node) = root {
            let val = node.borrow().val;

            if values.len() < depth {
                values.push(val);
            } else {
                values[depth - 1] = values[depth - 1].max(val);
            }

            Self::traverse(node.borrow().left.clone(), depth + 1, &mut values);
            Self::traverse(node.borrow().right.clone(), depth + 1, &mut values);
        }
    }
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = vec![];

        Self::traverse(root, 1, &mut values);

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
    fn test_largest_values() {
        assert_eq!(
            Solution::largest_values(tree![1, 3, 2, 5, 3, null, 9]),
            vec![1, 3, 9]
        );
        assert_eq!(Solution::largest_values(tree![1, 2, 3]), vec![1, 3]);
        assert_eq!(Solution::largest_values(tree![1]), vec![1]);
        assert_eq!(Solution::largest_values(tree![1, null, 2]), vec![1, 2]);
        assert_eq!(Solution::largest_values(tree![]), vec![]);
    }
}
