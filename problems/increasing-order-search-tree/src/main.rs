fn main() {
    assert_eq!(
        Solution::increasing_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 7,
                                    left: None,
                                    right: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 8,
                                        left: None,
                                        right: Some(Rc::new(RefCell::new(TreeNode {
                                            val: 9,
                                            left: None,
                                            right: None,
                                        }))),
                                    }))),
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        })))
    );

    assert_eq!(
        Solution::increasing_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
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
    fn helper(v: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            v.push(r.borrow().val);
            Solution::helper(v, r.borrow().left.clone());
            Solution::helper(v, r.borrow().right.clone());
        }
    }
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = vec![];
        Solution::helper(&mut v, root);
        v.sort_by(|a, b| b.cmp(a));
        let mut ans = None;
        for i in v {
            ans = Some(Rc::new(RefCell::new(TreeNode {
                val: i,
                left: None,
                right: ans,
            })));
        }
        return ans;
    }
}
