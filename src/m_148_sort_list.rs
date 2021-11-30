//Given the head of a linked list, return the list after sorting it in
//ascending order.
//
//
// Example 1:
//
//
//Input: head = [4,2,1,3]
//Output: [1,2,3,4]
//
//
// Example 2:
//
//
//Input: head = [-1,5,3,4,0]
//Output: [-1,0,3,4,5]
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
// The number of nodes in the list is in the range [0, 5 * 10⁴].
// -10⁵ <= Node.val <= 10⁵
//
//
//
// Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.
//e. constant space)?
// Related Topics Linked List Two Pointers Divide and Conquer Sorting Merge
//Sort 👍 5322 👎 196

use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l = ll![4, 2, 1, 3];
        assert_eq!(sort_list(l), ll![1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let l = ll![-1, 5, 3, 4, 0];
        assert_eq!(sort_list(l), ll![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(sort_list(None), None);
    }

    #[test]
    fn test_merge_sorted() {
        let l1 = ll![1, 2, 3];
        let l2 = ll![4, 5, 6];
        assert_eq!(merge_sorted(l1, l2), ll![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_2() {
        let l1 = ll![1, 3, 5];
        let l2 = ll![2, 4, 6];
        assert_eq!(merge_sorted(l1, l2), ll![1, 2, 3, 4, 5, 6]);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    merge_sort(head)
}

fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut acc: Vec<Option<Box<ListNode>>> = vec![];
    let mut current = head;
    while let Some(_) = current {
        let mut right = current.as_mut().unwrap().next.take();
        if let Some(_) = right  {
            let remain = (&mut right).as_mut().unwrap().next.take();
            acc.push(merge_sorted(current, right));
            current = remain;
        } else {
            acc.push(current);
            break
        }
    }

    while acc.len() > 1 {
        acc = fold_sorted(acc);
    }

    acc.pop().unwrap()
}

fn fold_sorted(mut v: Vec<Option<Box<ListNode>>>) -> Vec<Option<Box<ListNode>>> {
    if v.len() > 1 {
        let mut acc: Vec<Option<Box<ListNode>>> = vec![];
        while v.len() > 0 {
            if v.len() > 1 {
                acc.push(merge_sorted(v.pop().unwrap(), v.pop().unwrap()));
            } else {
                acc.push(v.pop().unwrap());
            }
        }
        acc
    } else {
        v
    }
}

fn merge_sorted(left: Option<Box<ListNode>>,
                right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (left, right) {
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

