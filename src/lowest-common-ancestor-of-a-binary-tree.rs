fn main() {
    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
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
            val: 3,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
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
            val: 5,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lowest_common_ancestor(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
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
            val: 1,
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
// https://www.geeksforgeeks.org/lowest-common-ancestor-binary-tree-set-1/
impl Solution {
    fn find_path(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        queue: &mut Vec<i32>,
    ) -> bool {
        if let Some(r) = root {
            queue.push(r.borrow().val);
            if r.borrow().val == val {
                return true;
            }
            if Solution::find_path(r.borrow().left.clone(), val, queue) {
                return true;
            }
            if Solution::find_path(r.borrow().right.clone(), val, queue) {
                return true;
            }
            queue.pop();
        }
        return false;
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p_queue = vec![];
        let mut q_queue = vec![];
        Solution::find_path(root.clone(), p.clone().unwrap().borrow().val, &mut p_queue);
        Solution::find_path(root.clone(), q.clone().unwrap().borrow().val, &mut q_queue);



        let mut i = 0;
        while i < p_queue.len() && i < q_queue.len() {
            if p_queue[i] != q_queue[i] {
                break;
            }
            i += 1;
        }

        return Some(Rc::new(RefCell::new(TreeNode::new(p_queue[i-1]))));
    }
}
