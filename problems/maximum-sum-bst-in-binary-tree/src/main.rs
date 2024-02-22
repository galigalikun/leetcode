fn main() {
    assert_eq!(
        Solution::max_sum_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
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
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))),
        20
    );
    assert_eq!(
        Solution::max_sum_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            }))),
            right: None
        })))),
        2
    );
    assert_eq!(
        Solution::max_sum_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: -4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -5,
                left: None,
                right: None
            })))
        })))),
        0
    );
    assert_eq!(
        Solution::max_sum_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: None,
                    right: None
                })))
            })))
        })))),
        25
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> (i32, i32, i32, i32) {
        if root.is_none() {
            return (1, 0, 0, 0);
        }
        let root = root.as_ref().unwrap().borrow();
        let (left_is_bst, left_sum, left_min, _left_max) = Self::dfs(&root.left, max);
        let (right_is_bst, right_sum, _right_min, right_max) = Self::dfs(&root.right, max);
        if left_is_bst == 1 && right_is_bst == 1 {
            if (root.left.is_none() || root.left.as_ref().unwrap().borrow().val < root.val)
                && (root.right.is_none() || root.right.as_ref().unwrap().borrow().val > root.val)
            {
                let sum = left_sum + right_sum + root.val;
                *max = std::cmp::max(*max, sum);
                return (1, sum, left_min, right_max);
            }
        }
        return (0, 0, 0, 0);
    }
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(&root, &mut max);
        return max;
    }
}
