fn main() {
    assert_eq!(
        Solution::insert_into_bst(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            5
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })))
    );

    assert_eq!(
        Solution::insert_into_bst(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 40,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 10,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 30,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 60,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 50,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 70,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            25
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 40,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 30,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 25,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 60,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 50,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 70,
                    left: None,
                    right: None,
                }))),
            }))),
        })))
    );

    assert_eq!(
        Solution::insert_into_bst(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            5
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })))
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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) => {
                let node_val = node.borrow().val;

                if val < node_val {
                    let left = node.borrow_mut().left.take();
                    let inserted_left = Solution::insert_into_bst(left, val);
                    node.borrow_mut().left = inserted_left;
                } else {
                    let right = node.borrow_mut().right.take();
                    let inserted_right = Solution::insert_into_bst(right, val);
                    node.borrow_mut().right = inserted_right;
                }

                Some(node)
            }
        }
    }
}
