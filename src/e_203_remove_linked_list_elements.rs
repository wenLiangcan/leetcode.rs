//Given the head of a linked list and an integer val, remove all the nodes of
//the linked list that has Node.val == val, and return the new head.
//
//
// Example 1:
//
//
//Input: head = [1,2,6,3,4,5,6], val = 6
//Output: [1,2,3,4,5]
//
//
// Example 2:
//
//
//Input: head = [], val = 1
//Output: []
//
//
// Example 3:
//
//
//Input: head = [7,7,7,7], val = 7
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 10⁴].
// 1 <= Node.val <= 50
// 0 <= val <= 50
//
// Related Topics Linked List Recursion 👍 3950 👎 144

use std::option::Option::Some;
use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(remove_elements(ll![1, 2, 6, 3, 4, 5, 6], 6), ll![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_2() {
        assert_eq!(remove_elements(None, 1), None);
    }

    #[test]
    fn test_3() {
        assert_eq!(remove_elements(ll![7, 7, 7, 7], 7), None);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head_holder = ListNode {
        next: head,
        val: 0
    };
    let mut current = &mut head_holder;
    while let Some(ref mut next) = current.next {
        if next.val == val {
            let mut matched = next;
            while let Some(ref next_next) = matched.next {
                if next_next.val == val {
                    matched = matched.next.as_mut().unwrap();
                } else {
                    break;
                }
            }
            let rest = matched.next.take();
            if let Some(_) = rest {
                current.next.replace(rest.unwrap());
                current = current.next.as_mut().unwrap();
            } else {
                current.next.take();
                break;
            }
        } else {
            current = current.next.as_mut().unwrap();
        }
    }
    head_holder.next
}
//leetcode submit region end(Prohibit modification and deletion)
