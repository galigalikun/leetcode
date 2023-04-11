fn main() {
    assert_eq!(
        Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            4,
            3
        ),
        false
    );

    assert_eq!(
        Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            4,
            3
        ),
        true
    );

    assert_eq!(
        Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            4,
            3
        ),
        false
    );
}

struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut root = root;
        let mut x = x;
        let mut y = y;
        let mut x_parent = None;
        let mut y_parent = None;
        let mut x_depth = 0;
        let mut y_depth = 0;
        let mut depth = 0;
        let mut queue = vec![];
        if let Some(root) = root {
            queue.push((root, None, depth));
        }
        while !queue.is_empty() {
            let (node, parent, depth) = queue.remove(0);
            if node.borrow().val == x {
                x_parent = parent;
                x_depth = depth;
            }
            if node.borrow().val == y {
                y_parent = parent.clone();
                y_depth = depth;
            }
            if let Some(left) = node.borrow().left.clone() {
                queue.push((left, Some(node), depth + 1));
            }
            if let Some(right) = node.borrow().right.clone() {
                queue.push((right, Some(node), depth + 1));
            }
        }
        if x_depth == y_depth && x_parent != y_parent {
            return true;
        }
        return false;
    }
}
