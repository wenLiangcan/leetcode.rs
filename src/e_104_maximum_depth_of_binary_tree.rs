//Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path
//from the root node down to the farthest leaf node.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: 3
//
//
// Example 2:
//
//
//Input: root = [1,null,2]
//Output: 2
//
//
// Example 3:
//
//
//Input: root = []
//Output: 0
//
//
// Example 4:
//
//
//Input: root = [0]
//Output: 1
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 10⁴].
// -100 <= Node.val <= 100
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 52
//72 👎 109

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n20 = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        };
        let n3 = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(n20)))
        };
        assert_eq!(max_depth(Some(Rc::new(RefCell::new(n3)))), 3);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_rec(root, 0)
}

fn max_depth_rec(root: Option<Rc<RefCell<TreeNode>>>, acc: i32) -> i32 {
    match root {
        Some(root) => {
            max_depth_rec((*root).borrow().left.as_ref().map(Rc::clone), acc + 1)
                .max(max_depth_rec((*root).borrow().right.as_ref().map(Rc::clone), acc + 1))
        },
        _ => acc
    }
}
//leetcode submit region end(Prohibit modification and deletion)
