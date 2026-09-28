fn main() {
    assert_eq!(
        Solution::flip_equiv(
            Some(Rc::new(RefCell::new(TreeNode {
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
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: None
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
            })))
        ),
        true
    );

    assert_eq!(Solution::flip_equiv(None, None), true);
    assert_eq!(
        Solution::flip_equiv(
            None,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })))
        ),
        false
    )
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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(n1), Some(n2)) => {
                let b1 = n1.borrow();
                let b2 = n2.borrow();

                if b1.val != b2.val {
                    return false;
                }

                let left1 = b1.left.clone();
                let right1 = b1.right.clone();
                let left2 = b2.left.clone();
                let right2 = b2.right.clone();

                let no_flip =
                    Solution::flip_equiv(left1.clone(), left2.clone()) && Solution::flip_equiv(right1.clone(), right2.clone());
                let flip = Solution::flip_equiv(left1, right2) && Solution::flip_equiv(right1, left2);

                no_flip || flip
            }
        }
    }
}
