fn main() {
    assert_eq!(
        Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        vec!["1->2->5", "1->3"]
    );
    assert_eq!(
        Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        vec!["1"]
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
    fn helper(
        result: &mut Vec<String>,
        s: String,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (left, right) {
            (Some(l), Some(r)) => {
                Solution::helper(
                    result,
                    format!("{}->{}", s, l.borrow().val),
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
                Solution::helper(
                    result,
                    format!("{}->{}", s, r.borrow().val),
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (Some(l), None) => {
                Solution::helper(
                    result,
                    format!("{}->{}", s, l.borrow().val),
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
            }
            (None, Some(r)) => {
                Solution::helper(
                    result,
                    format!("{}->{}", s, r.borrow().val),
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (None, None) => {
                result.push(s);
            }
        }
    }
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if let Some(r) = root {
            Solution::helper(
                &mut result,
                r.borrow().val.to_string(),
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }

        return result;
    }
}
