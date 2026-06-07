fn main() {
    assert_eq!(
        Solution::longest_univalue_path(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))
        })))),
        3
    );

    assert_eq!(
        Solution::longest_univalue_path(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            })))
        })))),
        2
    );
}

struct Solution {}
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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(node_rc) = root {
            let (node_val, left_child, right_child) = {
                let node = node_rc.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };

            let left_depth = Self::dfs(left_child.clone(), res);
            let right_depth = Self::dfs(right_child.clone(), res);

            let left_arrow = if let Some(left) = left_child {
                if left.borrow().val == node_val {
                    left_depth + 1
                } else {
                    0
                }
            } else {
                0
            };

            let right_arrow = if let Some(right) = right_child {
                if right.borrow().val == node_val {
                    right_depth + 1
                } else {
                    0
                }
            } else {
                0
            };

            *res = std::cmp::max(*res, left_arrow + right_arrow);
            std::cmp::max(left_arrow, right_arrow)
        } else {
            0
        }
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Solution::dfs(root, &mut res);
        res
    }
}
