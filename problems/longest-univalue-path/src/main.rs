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
        2
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
    fn dfs(root:Option<Rc<RefCell<TreeNode>>>, res:&mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = if let Some(left_node) = node.left.as_ref() {
                if left_node.borrow().val == node.val {
                    Self::dfs(node.left.clone(), res) + 1
                } else {
                    Self::dfs(node.left.clone(), res)
                }
            } else {
                0
            };
            let right = if let Some(right_node) = node.right.as_ref() {
                if right_node.borrow().val == node.val {
                    Self::dfs(node.right.clone(), res) + 1
                } else {
                    Self::dfs(node.right.clone(), res)
                }
            } else {
                0
            };
            *res = std::cmp::max(*res, left + right);
            std::cmp::max(left, right)
        } else {
            0
        }
    }
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        return Solution::dfs(root, &mut res);
    }
}
