fn main() {
    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None
            })))
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            })))
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None
        })))
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
// https://www.geeksforgeeks.org/lowest-common-ancestor-in-a-binary-search-tree/
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n1 = p.clone().unwrap().borrow().val;
        let n2 = q.clone().unwrap().borrow().val;
        if let Some(r) = root {
            if r.borrow().val > n1 && r.borrow().val > n2 {
                return Solution::lowest_common_ancestor(
                    r.borrow().left.clone(),
                    p.clone(),
                    q.clone(),
                );
            }
            if r.borrow().val < n1 && r.borrow().val < n2 {
                return Solution::lowest_common_ancestor(
                    r.borrow().right.clone(),
                    p.clone(),
                    q.clone(),
                );
            }

            return Some(Rc::new(RefCell::new(TreeNode::new(r.borrow().val))));
        }
        return None;
    }
}
