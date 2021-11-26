//Given the root of a binary tree, return the preorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [1,2,3]
//
//
// Example 2:
//
//
//Input: root = []
//Output: []
//
//
// Example 3:
//
//
//Input: root = [1]
//Output: [1]
//
//
// Example 4:
//
//
//Input: root = [1,2]
//Output: [1,2]
//
//
// Example 5:
//
//
//Input: root = [1,null,2]
//Output: [1,2]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
//
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
// Related Topics Stack Tree Depth-First Search Binary Tree 👍 3202 👎 109

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n3 = TreeNode::new(3);
        let mut n2 = TreeNode::new(2);
        n2.left = Some(Rc::new(RefCell::new(n3)));
        let mut n1 = TreeNode::new(1);
        n1.right = Some(Rc::new(RefCell::new(n2)));
        assert_eq!(preorder_traversal(Some(Rc::new(RefCell::new(n1)))), vec![1, 2, 3]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    match root {
        Some(mut current) => {
            let mut rights: Vec<Rc<RefCell<TreeNode>>> = vec![];
            loop {
                v.push((*current).borrow().val);
                let right = (*current).borrow().right.as_ref().map(Rc::clone);
                if (*current).borrow().left.is_some() {
                    if let Some(r) = right {
                        rights.push(r);
                    }
                    let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                    current = left;
                } else if let Some(r) = right {
                    current = r;
                } else {
                    if let Some(parent_right) = rights.pop() {
                        current = parent_right;
                    } else {
                        break;
                    }
                }
            }
        },
        _ => ()
    }
    v
}
//leetcode submit region end(Prohibit modification and deletion)
