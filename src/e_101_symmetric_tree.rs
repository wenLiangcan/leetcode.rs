//Given the root of a binary tree, check whether it is a mirror of itself (i.e.,
// symmetric around its center).
//
//
// Example 1:
//
//
//Input: root = [1,2,2,3,4,4,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [1,2,2,null,3,null,3]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Could you solve it both recursively and iteratively? Related
//Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 7866 👎 195

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l_n2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
        };
        let r_n2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        };
        let n1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(l_n2))),
            right: Some(Rc::new(RefCell::new(r_n2)))
        }));
        assert!(is_symmetric(Some(Rc::clone(&n1))));
        assert!(is_symmetric_rec(Some(Rc::clone(&n1))));
    }

    #[test]
    fn test_2() {
        let n2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        }));
        let n1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::clone(&n2)),
            right: Some(Rc::clone(&n2))
        }));
        assert!(!is_symmetric(Some(Rc::clone(&n1))));
        assert!(!is_symmetric_rec(Some(Rc::clone(&n1))));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(root) => {
            let mut l_current: Option<Rc<RefCell<TreeNode>>> = (*root).borrow().left.as_ref().map(Rc::clone);
            let mut r_current: Option<Rc<RefCell<TreeNode>>> = (*root).borrow().right.as_ref().map(Rc::clone);
            let mut l_rights: Vec<Rc<RefCell<TreeNode>>> = vec![];
            let mut r_lefts: Vec<Rc<RefCell<TreeNode>>> = vec![];
            loop {
                match (l_current, r_current) {
                    (None, None) => return true,
                    (Some(l), Some(r)) => {
                        if (*l).borrow().val != (*r).borrow().val {
                            return false;
                        } else {
                            let l_has_left = (*l).borrow().left.is_some();
                            let r_has_right = (*r).borrow().right.is_some();
                            if l_has_left != r_has_right {
                                return false;
                            } else {
                                let l_has_right = (*l).borrow().right.is_some();
                                let r_has_left = (*r).borrow().left.is_some();
                                if l_has_right != r_has_left {
                                    return false;
                                } else {
                                    if l_has_left && r_has_right {
                                        if l_has_right && r_has_left {
                                            l_rights.push(Rc::clone((*l).borrow().right.as_ref().unwrap()));
                                            r_lefts.push(Rc::clone((*r).borrow().left.as_ref().unwrap()));
                                        }
                                        l_current = (*l).borrow().left.as_ref().map(Rc::clone);
                                        r_current = (*r).borrow().right.as_ref().map(Rc::clone);
                                    } else {
                                        if l_has_right && r_has_left {
                                            l_current = (*l).borrow().right.as_ref().map(Rc::clone);
                                            r_current = (*r).borrow().left.as_ref().map(Rc::clone);
                                        } else {
                                            match (l_rights.pop(), r_lefts.pop()) {
                                                (None, None) => return true,
                                                (l_r@Some(_), r_l@Some(_)) => {
                                                    l_current = l_r;
                                                    r_current = r_l;
                                                },
                                                _ => return false
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => return false
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

pub fn is_symmetric_rec(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(root) => {
            is_symmetric_rec_inner((*root).borrow().left.as_ref().map(Rc::clone),
                                   (*root).borrow().right.as_ref().map(Rc::clone))
        }
    }
}

fn is_symmetric_rec_inner(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            if (*l).borrow().val != (*r).borrow().val {
                false
            } else {
                is_symmetric_rec_inner((*l).borrow().left.as_ref().map(Rc::clone),
                                       (*r).borrow().right.as_ref().map(Rc::clone))
                    && is_symmetric_rec_inner((*l).borrow().right.as_ref().map(Rc::clone),
                                              (*r).borrow().left.as_ref().map(Rc::clone))
            }
        },
        _ => false
    }
}
