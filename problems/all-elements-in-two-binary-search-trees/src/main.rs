fn main() {
    assert_eq!(
        Solution::get_all_elements(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            })))
        ),
        vec![0, 1, 1, 2, 3, 4]
    );

    assert_eq!(
        Solution::get_all_elements(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        ),
        vec![1, 1, 8, 8]
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
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        if let Some(r1) = root1 {
            if let Some(r2) = root2 {
                let mut v1 = Solution::get_all_elements(r1.borrow().left.clone(), None);
                v1.push(r1.borrow().val);
                let mut v2 = Solution::get_all_elements(None, r2.borrow().left.clone());
                v2.push(r2.borrow().val);
                v1.append(&mut v2);
                v1.append(&mut Solution::get_all_elements(
                    r1.borrow().right.clone(),
                    r2.borrow().right.clone(),
                ));
                return v1;
            } else {
                let mut v1 = Solution::get_all_elements(r1.borrow().left.clone(), None);
                v1.push(r1.borrow().val);
                v1.append(&mut Solution::get_all_elements(
                    r1.borrow().right.clone(),
                    None,
                ));
                return v1;
            }
        } else if let Some(r2) = root2 {
            let mut v2 = Solution::get_all_elements(None, r2.borrow().left.clone());
            v2.push(r2.borrow().val);
            v2.append(&mut Solution::get_all_elements(
                None,
                r2.borrow().right.clone(),
            ));
            return v2;
        }
        return vec![];
    }
}
