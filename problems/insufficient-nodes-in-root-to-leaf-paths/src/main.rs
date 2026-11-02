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
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 14,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))
    );
    assert_eq!(
        Solution::sufficient_subset(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -5,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
            }))),
            -1
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
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
        fn prune(
            node: Option<Rc<RefCell<TreeNode>>>,
            path_sum: i32,
            limit: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let node = node?;

            let (val, left, right) = {
                let mut current = node.borrow_mut();
                (current.val, current.left.take(), current.right.take())
            };
            let had_child = left.is_some() || right.is_some();

            let next_sum = path_sum + val;
            let left = prune(left, next_sum, limit);
            let right = prune(right, next_sum, limit);

            {
                let mut current = node.borrow_mut();
                current.left = left;
                current.right = right;

                if !had_child && next_sum < limit {
                    return None;
                }

                if had_child && current.left.is_none() && current.right.is_none() {
                    return None;
                }
            }

            Some(node)
        }

        prune(root, 0, limit)
    }
}
