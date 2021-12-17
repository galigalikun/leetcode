fn main() {
    assert_eq!(
        Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })))
        })))),
        3
    );

    assert_eq!(
        Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })))),
        1
    );

    assert_eq!(
        Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -7,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 0,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: -1,
                                    left: None,
                                    right: None,
                                }))),
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                    val: -4,
                                    left: None,
                                    right: None,
                                }))),
                                right: None,
                            }))),
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -7,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: -6,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -6,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 9,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                    val: -2,
                                    left: None,
                                    right: None,
                                }))),
                                right: None,
                            }))),
                            right: None,
                        }))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })))),
        8
    );
}
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
struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(depth: i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            return std::cmp::max(
                Solution::helper(depth + 1, r.borrow().left.clone()),
                Solution::helper(depth + 1, r.borrow().right.clone()),
            );
        }
        return depth;
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let left = Solution::helper(0, r.borrow().left.clone());
            let right = Solution::helper(0, r.borrow().right.clone());
            return left + right;
        }

        return 0;
    }
}
