//You are given the root node of a binary search tree (BST) and a value to
//insert into the tree. Return the root node of the BST after the insertion. It is
//guaranteed that the new value does not exist in the original BST.
//
// Notice that there may exist multiple valid ways for the insertion, as long
//as the tree remains a BST after insertion. You can return any of them.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3], val = 5
//Output: [4,2,7,1,3,5]
//Explanation: Another accepted tree is:
//
//
//
// Example 2:
//
//
//Input: root = [40,20,60,10,30,50,70], val = 25
//Output: [40,20,60,10,30,50,70,null,null,25]
//
//
// Example 3:
//
//
//Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
//Output: [4,2,7,1,3,5]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree will be in the range [0, 10⁴].
// -10⁸ <= Node.val <= 10⁸
// All the values Node.val are unique.
// -10⁸ <= val <= 10⁸
// It's guaranteed that val does not exist in the original BST.
//
// Related Topics Tree Binary Search Tree Binary Tree 👍 2194 👎 110
use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::e_94_binary_tree_inorder_traversal::inorder_traversal;
    use super::super::m_98_validate_binary_search_tree::is_valid_bst;

    #[test]
    fn test() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        })));
        let new_n = insert_into_bst(n, 5);
        assert!(is_valid_bst(new_n.as_ref().map(Rc::clone)));
        let mut items = inorder_traversal(new_n.as_ref().map(Rc::clone));
        items.sort();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn test_2() {
        let n = Rc::new(RefCell::new(TreeNode {
            val: 40,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(30))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 60,
                left: Some(Rc::new(RefCell::new(TreeNode::new(50)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(70))))
            })))
        }));
        let new_n = insert_into_bst(Some(Rc::clone(&n)), 25);
        assert!(is_valid_bst(new_n.as_ref().map(Rc::clone)));
        let mut items = inorder_traversal(new_n.as_ref().map(Rc::clone));
        items.sort();
        assert_eq!(items, vec![10, 20, 25, 30, 40, 50, 60, 70]);
    }

    #[test]
    fn test_3() {
        let n = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        }));
        let new_n = insert_into_bst(Some(Rc::clone(&n)), 5);
        assert!(is_valid_bst(new_n.as_ref().map(Rc::clone)));
        let mut items = inorder_traversal(new_n.as_ref().map(Rc::clone));
        items.sort();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 7]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    match root {
        None => node,
        Some(mut current) => {
            let root = Some(Rc::clone(&current));
            loop {
                if (*current).borrow().val > val {
                    if (*current).borrow().left.is_some() {
                        let left = (*current).borrow().left.as_ref().map(Rc::clone).unwrap();
                        current = left;
                    } else {
                        (*current).borrow_mut().left = node;
                        return root;
                    }
                } else {
                    if (*current).borrow().right.is_some() {
                        let mut right = (*current).borrow().right.as_ref().map(Rc::clone).unwrap();
                        current = right;
                    } else {
                        (*current).borrow_mut().right = node;
                        return root;
                    }
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
