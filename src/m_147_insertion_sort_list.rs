//Given the head of a singly linked list, sort the list using insertion sort,
//and return the sorted list's head.
//
// The steps of the insertion sort algorithm:
//
//
// Insertion sort iterates, consuming one input element each repetition and
//growing a sorted output list.
// At each iteration, insertion sort removes one element from the input data,
//finds the location it belongs within the sorted list and inserts it there.
// It repeats until no input elements remain.
//
//
// The following is a graphical example of the insertion sort algorithm. The
//partially sorted list (black) initially contains only the first element in the
//list. One element (red) is removed from the input data and inserted in-place into
//the sorted list with each iteration.
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
//
// Constraints:
//
//
// The number of nodes in the list is in the range [1, 5000].
// -5000 <= Node.val <= 5000
//
// Related Topics Linked List Sorting 👍 1318 👎 722

use leetcode::{ListNode, ll};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l = ll![4, 2, 1, 3];
        assert_eq!(insertion_sort_list(l), ll![1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let l = ll![-1, 5, 3, 4, 0];
        assert_eq!(insertion_sort_list(l), ll![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(insertion_sort_list(None), None);
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(n) => {
            let mut sorted: Option<Box<ListNode>> = None;
            let mut current = *n;
            loop {
                let next = current.next.take();
                sorted = insert_sorted(sorted, current);
                match next {
                    None => break,
                    Some(n) => {
                        current = *n;
                    }
                }
            }
            sorted
        }
    }
}

fn insert_sorted(head: Option<Box<ListNode>>, mut node: ListNode) -> Option<Box<ListNode>> {
    match head {
        None => Some(Box::new(node)),
        Some(mut n) => {
            let inserted = node.val;
            if inserted <= n.val {
                node.next = Some(n);
                Some(Box::new(node))
            } else {
                let mut current = &mut n;
                while let Some(ref next) = current.next {
                    if inserted <= next.val {
                        let tail = current.next.take();
                        node.next = tail;
                        current.next = Some(Box::new(node));
                        return Some(n);
                    } else {
                        current = current.next.as_mut().unwrap();
                    }
                }
                current.next.replace(Box::new(node));
                Some(n)
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
