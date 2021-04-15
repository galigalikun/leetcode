fn main() {
    assert_eq!(
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })))
    );
    assert_eq!(
        Solution::build_tree(vec![-1], vec![-1]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: None
        })))
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
// https://qiita.com/KueharX/items/c5442e4cd3baa6bb1ce0
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 || inorder.len() == 0 {
            return None;
        }
        let preorders_index = preorder[0];
        if let Some(inorders_index) = inorder.iter().position(|&x| x == preorders_index) {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: preorders_index,
                left: Solution::build_tree(
                    preorder[1..inorders_index + 1].to_vec(),
                    inorder[..inorders_index].to_vec(),
                ),
                right: Solution::build_tree(
                    preorder[inorders_index + 1..].to_vec(),
                    inorder[inorders_index + 1..].to_vec(),
                ),
            })));
        }

        println!("debug!!");
        return None;
    }
}
