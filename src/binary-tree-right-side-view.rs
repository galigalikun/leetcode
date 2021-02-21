fn main() {
    assert_eq!(
        Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
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
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            })))
        })))),
        vec![1, 3, 4]
    );

    assert_eq!(
        Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        vec![1, 3]
    );

    assert_eq!(Solution::right_side_view(None), vec![]);

    assert_eq!(
        Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: None
        })))),
        vec![1, 2]
    );

    /*
    1
    / \
    2 3
    /
    4
    */

    assert_eq!(
        Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
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
                right: None
            })))
        })))),
        vec![1, 3, 4]
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
        deep: usize,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(l) = left {
            let d = deep + 1;
            if let Some(n) = result.get_mut(d) {
                (*n).push(l.borrow().val);
            } else {
                result.push(vec![l.borrow().val]);
            }
            Solution::helper(
                result,
                d,
                l.borrow().left.clone(),
                l.borrow().right.clone(),
            );
        }
        if let Some(r) = right {
            let d = deep + 1;
            if let Some(n) = result.get_mut(d) {
                (*n).push(r.borrow().val);
            } else {
                result.push(vec![r.borrow().val]);
            }
            Solution::helper(
                result,
                d,
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let deep = 0;
        let result = &mut Vec::new();
        if let Some(r) = root {
            result.push(vec![r.borrow().val]);
            Solution::helper(
                result,
                deep,
                r.borrow().left.clone(),
                r.borrow().right.clone(),
            );
        }
        let mut a = Vec::new();
        for r in result {
            if let Some(&n) = r .last() {
                a.push(n);
            }
        }
        return a;
    }
}
