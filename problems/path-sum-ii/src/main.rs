fn main() {
    assert_eq!(
        Solution::path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 13,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                })))
            }))),
            22
        ),
        vec![[5, 4, 11, 2], [5, 8, 4, 5]]
    );

    // assert_eq!(
    //     Solution::path_sum(
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             val: 1,
    //             left: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 2,
    //                 left: None,
    //                 right: None,
    //             }))),
    //             right: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 3,
    //                 left: None,
    //                 right: None,
    //             }))),
    //         }))),
    //         5
    //     ),
    //     vec![]
    // );

    // assert_eq!(
    //     Solution::path_sum(
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             val: 1,
    //             left: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 2,
    //                 left: None,
    //                 right: None,
    //             }))),
    //             right: None,
    //         }))),
    //         0
    //     ),
    //     vec![]
    // );
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
        result: &mut Vec<Vec<i32>>,
        mut vec: Vec<i32>,
        target_sum: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match (left, right) {
            (Some(l), Some(r)) => {
                let mut v_l = vec.clone();
                v_l.push(l.borrow().val);
                Solution::helper(
                    result,
                    v_l,
                    target_sum,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );

                let mut v_r = vec.clone();
                v_r.push(r.borrow().val);
                Solution::helper(
                    result,
                    v_r,
                    target_sum,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (Some(l), None) => {
                vec.push(l.borrow().val);
                Solution::helper(
                    result,
                    vec,
                    target_sum,
                    l.borrow().left.clone(),
                    l.borrow().right.clone(),
                );
            }
            (None, Some(r)) => {
                vec.push(r.borrow().val);
                Solution::helper(
                    result,
                    vec,
                    target_sum,
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            (None, None) => {
                if target_sum == vec.iter().fold(0, |sum, a| sum + a) {
                    result.push(vec);
                }
            }
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(n) = root {
            let result = &mut vec![];
            Solution::helper(
                result,
                vec![n.borrow().val],
                target_sum,
                n.borrow().left.clone(),
                n.borrow().right.clone(),
            );

            return result.to_vec();
        }
        return vec![];
    }
}
