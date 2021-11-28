//Given the root of a binary tree, invert the tree, and return its root.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3,6,9]
//Output: [4,7,2,9,6,3,1]
//
//
// Example 2:
//
//
//Input: root = [2,1,3]
//Output: [2,3,1]
//
//
// Example 3:
//
//
//Input: root = []
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 69
//61 👎 95

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n4 = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
            })))
        };
        let n4_inverted = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
            })))
        };
        assert_eq!(invert_tree(Some(Rc::new(RefCell::new(n4)))), Some(Rc::new(RefCell::new(n4_inverted))));
    }

    #[test]
    fn test_2() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None
        })));
        let inverted = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
        })));
        assert_eq!(invert_tree(n), inverted);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => (),
        Some(ref r) => {
            let has_left = r.borrow().left.is_some();
            let has_right = r.borrow().right.is_some();

            if has_left || has_right {
                let left = r.borrow().left.as_ref().map(Rc::clone);
                let right = r.borrow().right.as_ref().map(Rc::clone);
                r.borrow_mut().left = right.as_ref().map(Rc::clone);
                r.borrow_mut().right = left.as_ref().map(Rc::clone);
                if has_left {
                    invert_tree(left);
                }
                if has_right {
                    invert_tree(right);
                }
            }
        }
    }
    root
}
//leetcode submit region end(Prohibit modification and deletion)
