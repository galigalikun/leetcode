fn main() {
    assert_eq!(
        Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }))),
            }))),
        })))
    );
    assert_eq!(
        Solution::bst_from_preorder(vec![1, 3]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })))
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
    fn insert(root: &mut TreeNode, val: i32) {
        if val < root.val {
            if let Some(left) = &mut root.left {
                Solution::insert(&mut left.borrow_mut(), val);
            } else {
                root.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        } else {
            if let Some(right) = &mut root.right {
                Solution::insert(&mut right.borrow_mut(), val);
            } else {
                root.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        }
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder.into_iter();
        let mut root = TreeNode::new(preorder.next().unwrap());
        for val in preorder {
            Self::insert(&mut root, val);
        }
        return Some(Rc::new(RefCell::new(root)));
    }
}
