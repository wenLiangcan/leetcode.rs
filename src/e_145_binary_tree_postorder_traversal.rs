//Given the root of a binary tree, return the postorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [3,2,1]
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
//Output: [2,1]
//
//
//
// Constraints:
//
//
// The number of the nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Recursive solution is trivial, could you do it iteratively?
//Related Topics Stack Tree Depth-First Search Binary Tree 👍 3378 👎 129

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
        assert_eq!(postorder_traversal(Some(Rc::new(RefCell::new(n1)))), vec![3, 2, 1]);
    }

    #[test]
    fn test_2() {
        let n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        let mut n3 = TreeNode::new(3);
        n3.left = Some(Rc::new(RefCell::new(n1)));
        n3.right = Some(Rc::new(RefCell::new(n2)));
        assert_eq!(postorder_traversal(Some(Rc::new(RefCell::new(n3)))), vec![1, 2, 3]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

enum Pos {
    Right,
    Root
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    match root {
        Some(mut current) => {
            let mut right_and_roots: Vec<(Pos, Rc<RefCell<TreeNode>>)> = vec![];
            'outer: loop {
                let right = (*current).borrow().right.as_ref().map(Rc::clone);
                if (*current).borrow().left.is_some() {
                    right_and_roots.push((Pos::Root, Rc::clone(&current)));
                    if let Some(ref r) = right {
                        right_and_roots.push((Pos::Right, Rc::clone(r)));
                    }
                    let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                    current = left;
                } else if let Some(ref r) = right {
                    right_and_roots.push((Pos::Root, Rc::clone(&current)));
                    current = Rc::clone(r);
                } else {
                    v.push((*current).borrow().val);
                    'inner: loop {
                        if let Some((pos, node)) = right_and_roots.pop() {
                            match pos {
                                Pos::Root => {
                                    v.push((*node).borrow().val);
                                    continue 'inner;
                                },
                                Pos::Right => {
                                    current = Rc::clone(&node);
                                    continue 'outer;
                                }
                            }
                        } else {
                            break 'outer;
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
