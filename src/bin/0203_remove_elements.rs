use leetcode_rust::util::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;

        loop {
            match ptr {
                None => break,
                Some(node) if node.val == val => {
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
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
            linked![1, 2, 3, 4, 5]
        );
    }
}
