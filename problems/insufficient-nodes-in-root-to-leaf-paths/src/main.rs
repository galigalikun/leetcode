fn main() {
    assert_eq!(
        Solution::sufficient_subset(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -99,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: -99,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -99,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -99,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 12,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 13,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: -99,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 14,
                            left: None,
                            right: None
                        })))
                    })))
                })))
            }))),
            1
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    })))
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 14,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))
        })))
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
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return None;
    }
}
