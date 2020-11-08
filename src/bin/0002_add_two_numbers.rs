use leetcode_rust::util::linked_list::ListNode;

struct Solution;

impl Solution {
    fn traverse_and_add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        ans: &mut Vec<i32>,
        carry: &mut i32,
    ) {
        let (l1next, l2next) = match (l1, l2) {
            (Some(a), Some(b)) => {
                let sum = a.val + b.val + *carry;
                *carry = sum / 10;
                ans.push(sum % 10);
                (a.next, b.next)
            }
            (Some(a), None) | (None, Some(a)) => {
                let sum = a.val + *carry;
                *carry = sum / 10;
                ans.push(sum % 10);
                (a.next, None)
            }
            (None, None) => {
                if *carry == 1 {
                    ans.push(1);
                    *carry = 0;
                }
                return;
            }
        };
        Self::traverse_and_add(l1next, l2next, ans, carry);
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use leetcode_rust::util::linked_list::to_list;
        let mut ans = vec![];

        Self::traverse_and_add(l1, l2, &mut ans, &mut 0);

        to_list(ans)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_rust::linked;
    use leetcode_rust::util::linked_list::to_list;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(linked!(2, 4, 3), linked!(5, 6, 4)),
            linked!(7, 0, 8)
        );
        assert_eq!(
            Solution::add_two_numbers(linked!(0), linked!(0)),
            linked!(0)
        );
        assert_eq!(
            Solution::add_two_numbers(linked!(9, 9, 9, 9, 9, 9, 9), linked!(9, 9, 9, 9)),
            linked!(8, 9, 9, 9, 0, 0, 0, 1)
        );
    }
}
