fn main() {
    assert_eq!(Solution::distance_k(Some(Rc::new(RefCell::new(TreeNode{
        val:3,
        left:Some(Rc::new(RefCell::new(TreeNode{
            val:5,
            left:Some(Rc::new(RefCell::new(TreeNode{
            val:6,
            left:None,
            right:None,
        }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
            val:2,
            left:Some(Rc::new(RefCell::new(TreeNode{
            val:7,
            left:None,
            right:None,
        }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
            val:4,
            left:None,
            right:None,
        }))),
        }))),
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:Some(Rc::new(RefCell::new(TreeNode{
            val:0,
            left:None,
            right:None,
        }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
            val:8,
            left:None,
            right:None,
        }))),
        })))
    }))), target: Some(Rc::new(RefCell::new(TreeNode{
            val:5,
            left:None,
            right:None,
        }))), 2), vec![7,4,1]);

    assert_eq!(Solution::distance_k(Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:None,
            right:None,
        }))), target: Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:None,
            right:None,
        }))), 3), vec![]);
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
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
    }
}
