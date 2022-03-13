fn main() {
    assert_eq!(
        Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        25
    );

    assert_eq!(
        Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            })))
        })))),
        1026
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
        result: &mut Vec<i32>,
        n: String,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (left, right) {
            (Some(l), Some(r)) => {
                Solution::helper(
                    result,
                    n.clone() + &l.borrow().val.to_string(),
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
                Solution::helper(
                    result,
                    n + &r.borrow().val.to_string(),
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (Some(l), None) => {
                Solution::helper(
                    result,
                    n + &l.borrow().val.to_string(),
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
            }
            (None, Some(r)) => {
                Solution::helper(
                    result,
                    n + &r.borrow().val.to_string(),
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (None, None) => {
                result.push(n.parse::<i32>().unwrap());
            }
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = root {
            let result = &mut vec![];
            Solution::helper(
                result,
                n.borrow().val.to_string(),
                n.borrow().left.clone(),
                n.borrow().right.clone(),
            );
            return result.iter().sum::<i32>();
        }

        return 0;
    }
}
