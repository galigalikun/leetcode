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
            5
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
        let mut x_parent = None;
        let mut y_parent = None;
        let mut x_depth = -1;
        let mut y_depth = -1;
        let mut queue = std::collections::VecDeque::new();

        if let Some(root) = root {
            queue.push_back((root, None, 0));
        }

        while let Some((node, parent, depth)) = queue.pop_front() {
            let value = node.borrow().val;

            if value == x {
                x_parent = parent;
                x_depth = depth;
            }
            if value == y {
                y_parent = parent;
                y_depth = depth;
            }

            if x_depth != -1 && y_depth != -1 {
                break;
            }

            let left = node.borrow().left.clone();
            if let Some(left) = left {
                let parent_value = Some(value);
                queue.push_back((left, parent_value, depth + 1));
            }

            let right = node.borrow().right.clone();
            if let Some(right) = right {
                let parent_value = Some(value);
                queue.push_back((right, parent_value, depth + 1));
            }
        }

        x_depth == y_depth && x_parent != y_parent
    }
}
