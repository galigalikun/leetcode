fn main() {
    assert_eq!(
        Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 30,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 36,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 36,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 35,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 33,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 21,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 26,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))
    );

    assert_eq!(
        Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))
        })))
    );

    assert_eq!(
        Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            })))
        })))
    );

    assert_eq!(
        Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        })))
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/transform-bst-sum-tree/
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
    fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(r) = root {
            Solution::helper(&mut r.borrow_mut().right, sum);
            *sum += r.borrow().val;
            r.borrow_mut().val = *sum;
            Solution::helper(&mut r.borrow_mut().left, sum);
        }
    }
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut ans = root;
        Solution::helper(&mut ans, &mut sum);
        return ans;
    }
}
