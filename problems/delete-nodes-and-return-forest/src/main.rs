fn main() {
    assert_eq!(
        Solution::del_nodes(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            vec![3, 5]
        ),
        vec![
            Some(Rc::new(RefCell::new(TreeNode {
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
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            }))),
        ]
    );

    assert_eq!(
        Solution::del_nodes(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            vec![3]
        ),
        vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        })))]
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
    fn del_nodes_sub(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        to_delete: &mut Vec<i32>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if root.is_none() {
            return;
        }
        let mut root = root.as_mut().unwrap().borrow_mut();
        if to_delete.len() == 0 {
            return;
        }
        if root.left.is_some() {
            let mut left = root.left.take();
            Solution::del_nodes_sub(&mut left, to_delete, result);
            root.left = left;
        }
        if root.right.is_some() {
            let mut right = root.right.take();
            Solution::del_nodes_sub(&mut right, to_delete, result);
            root.right = right;
        }
        if to_delete.len() == 0 {
            return;
        }
        if to_delete[to_delete.len() - 1] == root.val {
            to_delete.pop();
            if root.left.is_some() {
                result.push(root.left.take());
            }
            if root.right.is_some() {
                result.push(root.right.take());
            }
            *root = TreeNode::new(0);
        }
    }
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];
        let mut to_delete = to_delete;
        to_delete.sort();
        let mut to_delete = to_delete.iter().rev().map(|x| *x).collect::<Vec<i32>>();
        let mut root = root;
        Solution::del_nodes_sub(&mut root, &mut to_delete, &mut result);
        if root.is_some() {
            result.push(root);
        }
        return result;
    }
}
