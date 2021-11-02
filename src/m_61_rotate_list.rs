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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let l1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        let l2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_len_1() {
        let l = ListNode::new(1);
        assert_eq!(l.len(), 1);
    }

    #[test]
    fn test_len_3() {
        let l1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        assert_eq!(l1.len(), 3);
    }

    #[test]
    fn test_split_at() {
        let mut l1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        let l2 = l1.split_at(1);
        assert_eq!(l1.len(), 2);
        assert_eq!(l2, ListNode::new(3));
    }

    #[test]
    fn test_append() {
        let mut l = ListNode::new(1);
        let node = ListNode::new(2);
        l.append(node);
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn test_rotate_right() {
        let l = ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3)))
        };
        assert_eq!(*rotate_right(Some(Box::new(l)), 1).unwrap(), ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(2)))
        });
    }

    #[test]
    fn test_rotate_right_3() {
        let l = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        let l2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3)))
            }))
        };
        assert_eq!(rotate_right(Some(Box::new(l)), 300), Some(Box::new(l2)));
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl ListNode {
    fn len(&self) -> usize {
        let mut len = 1;
        let mut current = self;
        loop {
            if let Some(ref next) = current.next {
                current = next;
                len += 1;
            } else {
                return len;
            }
        }
    }

    fn split_at(&mut self, index: usize) -> ListNode {
        let mut current = self;
        for _ in 0 .. index {
            current = current.next.as_mut().unwrap();
        }
        *current.next.take().unwrap()
    }

    fn append(&mut self, node: ListNode) {
        let mut current = self;
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
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    match head {
        Some(mut old_head) if k > 0 => {
            let len = old_head.len();
            let r = k as usize % len;
            if r > 0 {
                let i = len - r;
                let mut new_head = old_head.split_at(i - 1);
                new_head.append(*old_head);
                Some(Box::new(new_head))
            } else {
                Some(old_head)
            }
        }
        h => h
    }
}
//leetcode submit region end(Prohibit modification and deletion)
