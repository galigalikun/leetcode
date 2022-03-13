fn main() {
    assert_eq!(
        Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode {
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
        3
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
        result: &mut Vec<bool>,
        depth: usize,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(l) = left {
            if result.len() <= depth {
                result.push(true);
            }
            Solution::helper(
                result,
                depth + 1,
                l.borrow().left.clone(),
                l.borrow().right.clone(),
            );
        }
        if let Some(r) = right {
            if result.len() <= depth {
                result.push(true);
            }
            Solution::helper(
                result,
                depth + 1,
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = &mut vec![];
        if let Some(n) = root {
            result.push(true);
            Solution::helper(result, 1, n.borrow().left.clone(), n.borrow().right.clone());
        }
        return result.len() as i32;
    }
}
