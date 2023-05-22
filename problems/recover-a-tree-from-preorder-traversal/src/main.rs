fn main() {
    assert_eq!(
        Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string()),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })))
            })))
        })))
    );
    assert_eq!(
        Solution::recover_from_preorder("1-2--3---4-5--6---7".to_string()),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
        })))
    );
    assert_eq!(
        Solution::recover_from_preorder("1-401--349---90--88".to_string()),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 401,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 349,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 90,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 88,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut i = 0;
        while i < traversal.len() {
            let mut level = 0;
            while i < traversal.len() && traversal.as_bytes()[i] == b'-' {
                level += 1;
                i += 1;
            }
            let mut val = 0;
            while i < traversal.len() && traversal.as_bytes()[i] != b'-' {
                val = val * 10 + (traversal.as_bytes()[i] - b'0') as i32;
                i += 1;
            }
            while stack.len() > level {
                stack.pop();
            }
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            if let Some(parent) = stack.last() {
                if parent.borrow().left.is_none() {
                    parent.borrow_mut().left = Some(node.clone());
                } else {
                    parent.borrow_mut().right = Some(node.clone());
                }
            }
            stack.push(node);
        }
        return stack.first().cloned();
    }
}
