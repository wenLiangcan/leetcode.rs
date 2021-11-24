//Given the head of a sorted linked list, delete all duplicates such that each
//element appears only once. Return the linked list sorted as well.
//
//
// Example 1:
//
//
//Input: head = [1,1,2]
//Output: [1,2]
//
//
// Example 2:
//
//
//Input: head = [1,1,2,3,3]
//Output: [1,2,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 300].
// -100 <= Node.val <= 100
// The list is guaranteed to be sorted in ascending order.
//
// Related Topics Linked List 👍 3625 👎 167

use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(delete_duplicates(ll![1, 1, 2]), ll![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(delete_duplicates(ll![1, 1, 2, 3, 3]), ll![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(delete_duplicates(ll![1, 2, 2]), ll![1, 2]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head_holder = ListNode {
        next: head,
        val: 0
    };
    let mut current = &mut head_holder;
    while let Some(ref next) = current.next {
        if next.next.is_some() {
            let mut next = current.next.as_mut().unwrap();
            let mut next_next = next.next.as_mut().unwrap();
            if next.val == next_next.val {
                while let Some(ref nnn) = next_next.next {
                    if next_next.val == nnn.val {
                        next_next = next_next.next.as_mut().unwrap();
                    } else {
                        break;
                    }
                }
                next.next = next_next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        } else {
            break;
        }
    }
    head_holder.next
}
//leetcode submit region end(Prohibit modification and deletion)
