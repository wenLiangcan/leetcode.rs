//Given a binary search tree (BST), find the lowest common ancestor (LCA) of
//two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor
//is defined between two nodes p and q as the lowest node in T that has both p
//and q as descendants (where we allow a node to be a descendant of itself).”
//
//
// Example 1:
//
//
//Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
//Output: 6
//Explanation: The LCA of nodes 2 and 8 is 6.
//
//
// Example 2:
//
//
//Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
//Output: 2
//Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant
//of itself according to the LCA definition.
//
//
// Example 3:
//
//
//Input: root = [2,1], p = 2, q = 1
//Output: 2
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [2, 10⁵].
// -10⁹ <= Node.val <= 10⁹
// All Node.val are unique.
// p != q
// p and q will exist in the BST.
//
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 4430
// 👎 160

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n8 = Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
        }));
        let n2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
            })))
        }));
        let n = Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::clone(&n2)),
            right: Some(Rc::clone(&n8))
        }));
        assert_eq!(lowest_common_ancestor(Some(Rc::clone(&n)), Some(Rc::clone(&n2)), Some(Rc::clone(&n8))), Some(Rc::clone(&n)));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p?.borrow().val;
    let q = q?.borrow().val;
    let mut current = root?;
    loop {
        let val = current.borrow().val;

        if val == p || val == q || (p < val && val < q) || (q < val && val < p) {
            return Some(current);
        } else if p < val && q < val {
            let left = current.borrow().left.as_ref().map(Rc::clone)?;
            current = left;
        } else if p > val && q > val {
            let right = current.borrow().right.as_ref().map(Rc::clone)?;
            current = right;
        } else {
            panic!()
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
