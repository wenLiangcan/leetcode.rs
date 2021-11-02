//Given a linked list, swap every two adjacent nodes and return its head. You
//must solve the problem without modifying the values in the list's nodes (i.e.,
//only nodes themselves may be changed.)
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4]
//Output: [2,1,4,3]
//
//
// Example 2:
//
//
//Input: head = []
//Output: []
//
//
// Example 3:
//
//
//Input: head = [1]
//Output: [1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 100].
// 0 <= Node.val <= 100
//
// Related Topics Linked List Recursion 👍 4808 👎 252

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        let l = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4)))
                }))
            }))
        };
        let swapped = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))
        };
        assert_eq!(swap_pairs(Some(Box::new(l))), Some(Box::new(swapped)));
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
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(bh) => match *bh {
            mut h @ ListNode {next: Some(_), ..} => {
                let third = h.next.as_mut().unwrap().next.take();
                let mut next =
                    if let Some(tail) = swap_pairs(third) {
                        h.next.replace(tail)
                    } else {
                        h.next.take()
                    };
                next.as_mut().unwrap().next = Some(Box::new(h));
                next
            }
            _ => Some(bh)
        }
        h => h
    }
}

// fn swap_pairs_loop(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

// }

//leetcode submit region end(Prohibit modification and deletion)
