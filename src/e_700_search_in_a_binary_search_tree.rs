//You are given the root of a binary search tree (BST) and an integer val.
//
// Find the node in the BST that the node's value equals val and return the
//subtree rooted with that node. If such a node does not exist, return null.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3], val = 2
//Output: [2,1,3]
//
//
// Example 2:
//
//
//Input: root = [4,2,7,1,3], val = 5
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 5000].
// 1 <= Node.val <= 10⁷
// root is a binary search tree.
// 1 <= val <= 10⁷
//
// Related Topics Tree Binary Search Tree Binary Tree 👍 2090 👎 137

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        }));
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::clone(&n2)),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        })));
        assert_eq!(search_bst(n, 2), Some(Rc::clone(&n2)));
    }

    #[test]
    fn test_2() {
        let n2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        }));
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::clone(&n2)),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        })));
        assert_eq!(search_bst(n, 5), None);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;
use core::cmp::Ordering;

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(mut current) => {
            loop {
                let current_val = (*current).borrow().val;
                match current_val.cmp(&val) {
                    Ordering::Equal => return Some(Rc::clone(&current)),
                    Ordering::Greater if (*current).borrow().left.is_some() => {
                        let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                        current = left;
                    },
                    Ordering::Less if (*current).borrow().right.is_some() => {
                        let right = Rc::clone((*current).borrow().right.as_ref().unwrap());
                        current = right;
                    },
                    _ => return None
                };
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
