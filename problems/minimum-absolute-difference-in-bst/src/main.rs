fn main() {
    assert_eq!(
        Solution::get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None
            })))
        })))),
        1
    );

    assert_eq!(
        Solution::get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 48,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 12,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 49,
                    left: None,
                    right: None
                })))
            })))
        })))),
        1
    );

    assert_eq!(
        Solution::get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode {
            val: 236,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 104,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 227,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 701,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 911,
                    left: None,
                    right: None
                })))
            })))
        })))),
        9
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
    fn helper(ans: &mut i32, lst: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            for v in lst.clone() {
                *ans = std::cmp::min(*ans, (v - r.borrow().val).abs());
            }
            lst.push(r.borrow().val);
            Solution::helper(ans, lst, r.borrow().left.clone());
            Solution::helper(ans, lst, r.borrow().right.clone());
        }
    }
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = std::i32::MAX;
        Solution::helper(&mut ans, &mut vec![], root);
        return ans;
    }
}
