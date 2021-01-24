fn main() {
    assert_eq!(
        Solution::level_order_bottom(Some(Rc::new(RefCell::new(TreeNode {
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
        vec![vec![15, 7], vec![9, 20], vec![3]]
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
            Solution::helper(
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
            Solution::helper(
                result,
                level + 1,
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }
    }
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let result = &mut vec![];
        if let Some(r) = root {
            result.push(vec![r.borrow().val]);
            Solution::helper(result, 1, r.borrow().left.clone(), r.borrow().right.clone());
        }
        return result
            .into_iter()
            .map(|x| x.to_vec())
            .rev()
            .collect::<Vec<_>>();
    }
}
