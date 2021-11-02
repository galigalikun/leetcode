fn main() {
    assert_eq!(
        Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                })))
            })))
        })))),
        2
    );

    assert_eq!(
        Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode {
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
                            right: None
                        })))
                    })))
                })))
            })))
        })))),
        5
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
impl Solution {
    fn helper(
        depth: &mut Vec<usize>,
        level: usize,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (left, right) {
            (Some(l), Some(r)) => {
                Solution::helper(
                    depth,
                    level + 1,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
                Solution::helper(
                    depth,
                    level + 1,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (Some(l), None) => {
                Solution::helper(
                    depth,
                    level + 1,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
            }
            (None, Some(r)) => {
                Solution::helper(
                    depth,
                    level + 1,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (None, None) => {
                depth.push(level);
            }
        }
    }
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let depth = &mut vec![];
        if let Some(r) = root {
            Solution::helper(depth, 1, r.borrow().left.clone(), r.borrow().right.clone());
            let min = depth.iter().min().unwrap();
            return (*min) as i32;
        }

        return 0;
    }
}
