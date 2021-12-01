//Given the root of a Binary Search Tree and a target number k, return true if
//there exist two elements in the BST such that their sum is equal to the given
//target.
//
//
// Example 1:
//
//
//Input: root = [5,3,6,2,4,null,7], k = 9
//Output: true
//
//
// Example 2:
//
//
//Input: root = [5,3,6,2,4,null,7], k = 28
//Output: false
//
//
// Example 3:
//
//
//Input: root = [2,1,3], k = 4
//Output: true
//
//
// Example 4:
//
//
//Input: root = [2,1,3], k = 1
//Output: false
//
//
// Example 5:
//
//
//Input: root = [2,1,3], k = 3
//Output: true
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// -10⁴ <= Node.val <= 10⁴
// root is guaranteed to be a valid binary search tree.
// -10⁵ <= k <= 10⁵
//
// Related Topics Hash Table Two Pointers Tree Depth-First Search Breadth-First
//Search Binary Search Tree Binary Tree 👍 2933 👎 184

use leetcode::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
            })))
        })));
        assert!(find_target(n, 9));
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    match root {
        None => false,
        Some(mut current) => {
            let mut set: HashSet<i32> = HashSet::new();
            let mut rights: Vec<Rc<RefCell<TreeNode>>> = vec![];
            loop {
                let val = (*current).borrow().val;
                let diff = k - val;
                if set.contains(&diff) {
                    return true;
                }
                set.insert(val);
                let right = (*current).borrow().right.as_ref().map(Rc::clone);
                if (*current).borrow().left.is_some() {
                    if let Some(r) = right {
                        rights.push(r);
                    }
                    let left = (*current).borrow().left.as_ref().map(Rc::clone).unwrap();
                    current = left;
                } else if let Some(r) = right {
                    current = r;
                } else {
                    if let Some(r) = rights.pop() {
                        current = r;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
