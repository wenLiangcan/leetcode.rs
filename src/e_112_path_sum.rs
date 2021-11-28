//Given the root of a binary tree and an integer targetSum, return true if the
//tree has a root-to-leaf path such that adding up all the values along the path
//equals targetSum.
//
// A leaf is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
//Output: true
//Explanation: The root-to-leaf path with the target sum is shown.
//
//
// Example 2:
//
//
//Input: root = [1,2,3], targetSum = 5
//Output: false
//Explanation: There two root-to-leaf paths in the tree:
//(1 --> 2): The sum is 3.
//(1 --> 3): The sum is 4.
//There is no root-to-leaf path with sum = 5.
//
//
// Example 3:
//
//
//Input: root = [], targetSum = 0
//Output: false
//Explanation: Since the tree is empty, there are no root-to-leaf paths.
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 5000].
// -1000 <= Node.val <= 1000
// -1000 <= targetSum <= 1000
//
// Related Topics Tree Depth-First Search Binary Tree 👍 4359 👎 715

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                })))
            })))
        })));
        assert!(has_path_sum(n, 22));
    }

    #[test]
    fn test_2() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        })));
        assert!(!has_path_sum(n, 5));
    }

    #[test]
    fn test_3() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None
        })));
        assert!(!has_path_sum(n, 1));
    }

    #[test]
    fn test_4() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
                right: None
            })))
        })));
        assert!(!has_path_sum(n, 3));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(mut current) => {
            let mut path: Vec<i32> = vec![];
            let mut rights: Vec<(usize, Rc<RefCell<TreeNode>>)> = vec![];
            let mut level = 0usize;
            loop {
                path.push((*current).borrow().val);
                let right = (*current).borrow().right.as_ref().map(Rc::clone);
                if (*current).borrow().left.is_some() {
                    level += 1;
                    if let Some(ref r) = right {
                        rights.push((level, Rc::clone(r)));
                    }
                    let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                    current = left;
                } else if let Some(ref r) = right {
                    level += 1;
                    current = Rc::clone(r);
                } else {
                    if path.iter().sum::<i32>() == target_sum {
                        return true;
                    }
                    match rights.pop() {
                        None => return false,
                        Some((l, ref r)) => {
                            path.truncate(l);
                            level = l;
                            current = Rc::clone(r);
                        }
                    }
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
