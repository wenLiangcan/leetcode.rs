//Merge two sorted linked lists and return it as a sorted list. The list should
//be made by splicing together the nodes of the first two lists.
//
//
// Example 1:
//
//
//Input: l1 = [1,2,4], l2 = [1,3,4]
//Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
//
//Input: l1 = [], l2 = []
//Output: []
//
//
// Example 3:
//
//
//Input: l1 = [], l2 = [0]
//Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both l1 and l2 are sorted in non-decreasing order.
//
// Related Topics Linked List Recursion 👍 8867 👎 885

use leetcode::{ListNode, ll};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(merge_two_lists(ll![1, 2, 4], ll![1, 3, 4]), ll![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(merge_two_lists(None, None), None);
    }

    #[test]
    fn test_3() {
        assert_eq!(merge_two_lists(None, ll![0]), ll![0]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut l_n), Some(mut r_n)) => {
            let mut head = ListNode::new(i32::MIN);
            let mut ending = &mut head;

            let mut l_current = &mut l_n;
            let mut r_current = &mut r_n;

            loop {
                if l_current.val > r_current.val {
                    while let Some(ref r_next) = r_current.next {
                        if l_current.val > r_next.val {
                            r_current = r_current.next.as_mut().unwrap();
                        } else {
                            break;
                        }
                    }
                    let r_tail = r_current.next.take();
                    ending.next.replace(r_n);
                    ending = get_ending(ending);
                    if let Some(r) = r_tail {
                        r_n = r;
                        r_current = &mut r_n;
                    } else {
                        ending.next.replace(l_n);
                        return head.next;
                    }
                } else {
                    while let Some(ref l_next) = l_current.next {
                        if r_current.val > l_next.val {
                            l_current = l_current.next.as_mut().unwrap();
                        } else {
                            break;
                        }
                    }
                    let l_tail = l_current.next.take();
                    ending.next.replace(l_n);
                    ending = get_ending(ending);
                    if let Some(l) = l_tail {
                        l_n = l;
                        l_current = &mut l_n;
                    } else {
                        ending.next.replace(r_n);
                        return head.next;
                    }
                }
            }
        },
        (None, right) => right,
        (left, None) => left,
    }
}

fn get_ending(head: &mut ListNode) -> &mut ListNode {
    let mut current = head;
    while let Some(_) = current.next {
        current = current.next.as_mut().unwrap();
    }
    current
}
//leetcode submit region end(Prohibit modification and deletion)
