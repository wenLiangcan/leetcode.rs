//Given the root of a binary tree, determine if it is a valid binary search
//tree (BST).
//
// A valid BST is defined as follows:
//
//
// The left subtree of a node contains only nodes with keys less than the
//node's key.
// The right subtree of a node contains only nodes with keys greater than the
//node's key.
// Both the left and right subtrees must also be binary search trees.
//
//
//
// Example 1:
//
//
//Input: root = [2,1,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [5,1,4,null,null,3,6]
//Output: false
//Explanation: The root node's value is 5 but its right child's value is 4.
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// -2³¹ <= Node.val <= 2³¹ - 1
//
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 7991
// 👎 789

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
            })))
        })));
        assert!(!is_valid_bst(n));
    }

    #[test]
    fn test_2() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
        })));
        assert!(!is_valid_bst(n));
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_inner(root, None, None)
}

fn is_valid_bst_inner(root: Option<Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    match root {
        None => true,
        Some(current) => {
            let val = (*current).borrow().val;
            if let Some(ref left) = current.borrow().left {
                if !(left.borrow().val < val) {
                    return false;
                }
                if let Some(l) = lower {
                    if !(left.borrow().val > l) {
                        return false;
                    }
                }
            }
            if let Some(ref right) = current.borrow().right {
                if !(right.borrow().val > val) {
                    return false;
                }
                if let Some(u) = upper {
                    if !(right.borrow().val < u) {
                        return false;
                    }
                }
            }
            return is_valid_bst_inner((*current).borrow().left.as_ref().map(Rc::clone), lower, Some(val))
                && is_valid_bst_inner((*current).borrow().right.as_ref().map(Rc::clone), Some(val), upper);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
