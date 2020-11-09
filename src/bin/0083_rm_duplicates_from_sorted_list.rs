use leetcode_rust::util::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::HashSet;
        let mut ptr = &mut head;
        let mut vals = HashSet::new();

        loop {
            match ptr {
                None => break,
                Some(node) if !vals.insert(node.val) => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }

        head
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::linked;
    use leetcode_rust::util::linked_list::to_list;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(Solution::delete_duplicates(linked!(1, 1, 2)), linked!(1, 2));
        assert_eq!(
            Solution::delete_duplicates(linked!(1, 1, 2, 3, 3)),
            linked!(1, 2, 3)
        );
    }
}
