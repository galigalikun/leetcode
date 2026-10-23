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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, min_on_path: i32, max_on_path: i32) -> i32 {
        let Some(node) = root else {
            return max_on_path - min_on_path;
        };

        let node = node.borrow();
        let next_min = min_on_path.min(node.val);
        let next_max = max_on_path.max(node.val);

        let left = Self::dfs(node.left.clone(), next_min, next_max);
        let right = Self::dfs(node.right.clone(), next_min, next_max);
        left.max(right)
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root.clone() else {
            return 0;
        };

        let value = node.borrow().val;
        Self::dfs(root, value, value)
    }
}
