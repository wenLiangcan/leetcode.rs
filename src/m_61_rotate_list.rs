//Given the head of a linked list, rotate the list to the right by k places.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], k = 2
//Output: [4,5,1,2,3]
//
//
// Example 2:
//
//
//Input: head = [0,1,2], k = 4
//Output: [2,0,1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 500].
// -100 <= Node.val <= 100
// 0 <= k <= 2 * 10⁹
//
// Related Topics Linked List Two Pointers 👍 3327 👎 1205

use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let l1 = ll![1, 2, 3];
        let l2 = ll![1, 2, 3];
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_len_1() {
        let l = ListNode::new(1);
        assert_eq!(len(&l), 1);
    }

    #[test]
    fn test_len_3() {
        let l1 = ll![1, 2, 3];
        assert_eq!(len(&l1.unwrap()), 3);
    }

    #[test]
    fn test_split_at() {
        let mut l1 = ll![1, 2, 3].unwrap();
        let l2 = split_at(&mut l1, 1);
        assert_eq!(len(&l1), 2);
        assert_eq!(l2, ListNode::new(3));
    }

    #[test]
    fn test_append() {
        let mut l = ListNode::new(1);
        let node = ListNode::new(2);
        append(&mut l, node);
        assert_eq!(len(&l), 2);
    }

    #[test]
    fn test_rotate_right() {
        let l = ll![2, 3];
        assert_eq!(*rotate_right(l, 1).unwrap(), ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(2)))
        });
    }

    #[test]
    fn test_rotate_right_3() {
        let l = ll![1, 2, 3];
        let l2 = ll![1, 2, 3];
        assert_eq!(rotate_right(l, 300), l2);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

fn len(head: &ListNode) -> usize {
    let mut len = 1;
    let mut current = head;
    loop {
        if let Some(ref next) = current.next {
            current = next;
            len += 1;
        } else {
            return len;
        }
    }
}

fn split_at(head: &mut ListNode, index: usize) -> ListNode {
    let mut current = head;
    for _ in 0 .. index {
        current = current.next.as_mut().unwrap();
    }
    *current.next.take().unwrap()
}

fn append(head: &mut ListNode, node: ListNode) {
    let mut current = head;
    loop {
        if let Some(ref mut next) = current.next {
            current = next;
        } else {
            let node = Some(Box::new(node));
            current.next = node;
            break;
        }
    }
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    match head {
        Some(mut old_head) if k > 0 => {
            let len = len(&*old_head);
            let r = k as usize % len;
            if r > 0 {
                let i = len - r;
                let mut new_head = split_at(&mut *old_head, i - 1);
                append(&mut new_head, *old_head);
                Some(Box::new(new_head))
            } else {
                Some(old_head)
            }
        }
        h => h
    }
}
//leetcode submit region end(Prohibit modification and deletion)
