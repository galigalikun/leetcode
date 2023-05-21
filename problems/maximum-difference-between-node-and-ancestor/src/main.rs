fn main() {
    assert_eq!(
        Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 14,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 13,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })))),
        7
    );

    assert_eq!(
        Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })))),
        3
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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_: &mut i32) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left_min, left_max) = Self::dfs(node.left.clone(), max_);
            let (right_min, right_max) = Self::dfs(node.right.clone(), max_);
            let min = left_min.min(right_min);
            let max = left_max.max(right_max);
            *max_ = (*max_).max((node.val - min).abs());
            return (min.min(node.val), max.max(node.val));
        }
        return (i32::MAX, i32::MIN);
    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(root, &mut max);
        return max;
    }
}
