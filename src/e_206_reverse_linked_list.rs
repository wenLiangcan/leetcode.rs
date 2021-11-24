//Given the head of a singly linked list, reverse the list, and return the
//reversed list.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5]
//Output: [5,4,3,2,1]
//
//
// Example 2:
//
//
//Input: head = [1,2]
//Output: [2,1]
//
//
// Example 3:
//
//
//Input: head = []
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000
//
//
//
// Follow up: A linked list can be reversed either iteratively or recursively.
//Could you implement both?
// Related Topics Linked List Recursion 👍 9299 👎 166

use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(reverse_list(ll![1, 2, 3, 4, 5]), ll![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_list(ll![1, 2]), ll![2, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse_list(None), None);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut new_head: Option<Box<ListNode>> = None;
    while let Some(mut node) = current {
        current = node.next.take();
        node.next = new_head;
        new_head = Some(node);
    }
    new_head
}
//leetcode submit region end(Prohibit modification and deletion)
