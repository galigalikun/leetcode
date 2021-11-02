fn main() {
    assert_eq!(
        Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
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
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 13,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        })))
                    })))
                })))
            }))),
            22
        ),
        true
    );

    assert_eq!(
        Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            5
        ),
        false
    );

    assert_eq!(
        Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            0
        ),
        false
    );
}

pub struct Solution {}
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
    fn helper(
        result: &mut Vec<i32>,
        sum: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (left, right) {
            (Some(l), Some(r)) => {
                Solution::helper(
                    result,
                    sum + l.borrow().val,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
                Solution::helper(
                    result,
                    sum + r.borrow().val,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (Some(l), None) => {
                Solution::helper(
                    result,
                    sum + l.borrow().val,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
            }
            (None, Some(r)) => {
                Solution::helper(
                    result,
                    sum + r.borrow().val,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (None, None) => {
                result.push(sum);
            }
        }
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(n) = root {
            let result = &mut vec![];
            Solution::helper(
                result,
                n.borrow().val,
                n.borrow().left.clone(),
                n.borrow().right.clone(),
            );
            return result.iter().find(|&&x| x == target_sum) == Some(&target_sum);
        }

        return false;
    }
}
