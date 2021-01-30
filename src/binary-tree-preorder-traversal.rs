fn main() {
    assert_eq!(
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))),
        vec![1, 2, 3]
    );

    assert_eq!(Solution::preorder_traversal(None), vec![]);

    assert_eq!(
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        vec![1]
    );

    assert_eq!(
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
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

    assert_eq!(
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            })))
        })))),
        vec![1, 2]
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
        result: &mut Vec<i32>,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(l) = left {
            result.push(l.borrow().val);
            Solution::helper(result, l.borrow().left.clone(), l.borrow().right.clone());
        }
        if let Some(r) = right {
            result.push(r.borrow().val);
            Solution::helper(result, r.borrow().left.clone(), r.borrow().right.clone());
        }
    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let result = &mut vec![];
        if let Some(n) = root {
            result.push(n.borrow().val);
            Solution::helper(result, n.borrow().left.clone(), n.borrow().right.clone());
        }
        return result.to_vec();
    }
}
