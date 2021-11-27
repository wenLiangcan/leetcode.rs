//Given the root of a binary tree, return the inorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [1,3,2]
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
//Output: [2,1]
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
//Follow up: Recursive solution is trivial, could you do it iteratively?
//Related Topics Stack Tree Depth-First Search Binary Tree 👍 6218 👎 263

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
        assert_eq!(inorder_traversal(Some(Rc::new(RefCell::new(n1)))), vec![1, 3, 2]);
    }

    fn test_2() {
        let n1 = TreeNode::new(1);
        let mut n3 = TreeNode::new(3);
        n3.left = Some(Rc::new(RefCell::new(n1)));
        let mut n2 = TreeNode::new(2);
        n2.left = Some(Rc::new(RefCell::new(n3)));
        assert_eq!(inorder_traversal(Some(Rc::new(RefCell::new(n2)))), vec![1, 3, 2]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    match root {
        Some(mut current) => {
            let mut roots: Vec<Rc<RefCell<TreeNode>>> = vec![];
            'outer: loop {
                if (*current).borrow().left.is_some() {
                    roots.push(Rc::clone(&current));
                    let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                    current = left;
                } else {
                    v.push((*current).borrow().val);
                    if (*current).borrow().right.is_some() {
                        let right = Rc::clone((*current).borrow().right.as_ref().unwrap());
                        current = right;
                    } else {
                        'inner: loop {
                            if let Some(root) = roots.pop() {
                                v.push((*root).borrow().val);
                                if (*root).borrow().right.is_some() {
                                    current = Rc::clone((*root).borrow().right.as_ref().unwrap());
                                    continue 'outer;
                                } else {
                                    continue 'inner;
                                }
                            } else {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        },
        _ => ()
    }
    v
}
//leetcode submit region end(Prohibit modification and deletion)
