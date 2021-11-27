//Given the root of a binary tree, return the level order traversal of its
//nodes' values. (i.e., from left to right, level by level).
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: [[3],[9,20],[15,7]]
//
//
// Example 2:
//
//
//Input: root = [1]
//Output: [[1]]
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
// The number of nodes in the tree is in the range [0, 2000].
// -1000 <= Node.val <= 1000
//
// Related Topics Tree Breadth-First Search Binary Tree 👍 6425 👎 127

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
        assert_eq!(level_order(Some(Rc::new(RefCell::new(n3)))), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

enum Pos {
    Right(usize),
    Root(usize)
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = vec![];
    match root {
        Some(mut current) => {
            let mut level = 0usize;
            let mut right_and_roots: Vec<(Pos, Rc<RefCell<TreeNode>>)> = vec![];
            'outer: loop {
                if v.len() < level + 1 {
                    v.push(vec![]);
                }
                let right = (*current).borrow().right.as_ref().map(Rc::clone);
                if (*current).borrow().left.is_some() {
                    right_and_roots.push((Pos::Root(level), Rc::clone(&current)));
                    level += 1;
                    if let Some(ref r) = right {
                        right_and_roots.push((Pos::Right(level), Rc::clone(r)));
                    }
                    let left = Rc::clone((*current).borrow().left.as_ref().unwrap());
                    current = left;
                } else if let Some(ref r) = right {
                    right_and_roots.push((Pos::Root(level), Rc::clone(&current)));
                    level += 1;
                    current = Rc::clone(r);
                } else {
                    v[level].push((*current).borrow().val);
                    'inner: loop {
                        if let Some((pos, node)) = right_and_roots.pop() {
                            match pos {
                                Pos::Root(l) => {
                                    if v.len() < l + 1 {
                                        v.push(vec![]);
                                    }
                                    v[l].push((*node).borrow().val);
                                    continue 'inner;
                                },
                                Pos::Right(l) => {
                                    level = l;
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
