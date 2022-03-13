fn main() {
    assert_eq!(
        Solution::path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -2,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            8
        ),
        3
    );

    assert_eq!(
        Solution::path_sum(
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
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        }))),
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
        3
    );
}

struct Solution {}
// https://qiita.com/KueharX/items/de5f70a64667cfb04442
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
        result: &mut i32,
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        mut sum: Option<i32>,
    ) {
        if let Some(r) = root {
            sum = if let Some(s) = sum {
                Some(s + r.borrow().val)
            } else {
                Solution::helper(result, r.borrow().left.clone(), target_sum, None);
                Solution::helper(result, r.borrow().right.clone(), target_sum, None);
                Some(r.borrow().val)
            };

            if Some(target_sum) == sum {
                *result += 1;
            }
            Solution::helper(result, r.borrow().left.clone(), target_sum, sum);
            Solution::helper(result, r.borrow().right.clone(), target_sum, sum);
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut result = 0;
        Solution::helper(&mut result, root, target_sum, None);
        return result;
    }
}
