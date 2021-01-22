fn main() {
    assert_eq!(
        Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode {
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
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );

    assert_eq!(
        Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            })))
        })))),
        vec![vec![1], vec![3, 2], vec![4, 5]]
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
    fn both(
        result: &mut Vec<Vec<i32>>,
        level: usize,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(l) = left {
            if result.len() > level {
                result[level].push(l.borrow().val);
            } else {
                result.push(vec![l.borrow().val]);
            }
            Solution::both(
                result,
                level + 1,
                l.borrow().left.clone(),
                l.borrow().right.clone(),
            );
        }
        if let Some(r) = right {
            if result.len() > level {
                result[level].push(r.borrow().val);
            } else {
                result.push(vec![r.borrow().val]);
            }
            Solution::both(
                result,
                level + 1,
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }
    }
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let result = &mut vec![];
        if let Some(r) = root {
            result.push(vec![r.borrow().val]);
            Solution::both(result, 1, r.borrow().left.clone(), r.borrow().right.clone());
            for i in 0..result.len() {
                if i % 2 == 1 {
                    result[i] = result[i].iter().rev().map(|&x| x).collect::<Vec<_>>();
                }
            }
        }
        return result.to_vec();
    }
}
