fn main() {
    assert_eq!(
        Solution::balance_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        })))
    );

    assert_eq!(
        Solution::balance_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
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
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
        if let Some(root) = root {
            Solution::inorder_traversal(root.borrow().left.clone(), nodes);
            nodes.push(root.borrow().val);
            Solution::inorder_traversal(root.borrow().right.clone(), nodes);
        }
    }
    fn build_bst(nodes: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.is_empty() {
            return None;
        }
        let mid = nodes.len() / 2;
        let mut root = TreeNode::new(nodes[mid]);
        root.left = Solution::build_bst(&nodes[..mid].to_vec());
        root.right = Solution::build_bst(&nodes[mid + 1..].to_vec());
        return Some(Rc::new(RefCell::new(root)));
    }
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = vec![];
        Solution::inorder_traversal(root, &mut nodes);
        return Solution::build_bst(&nodes);
    }
}
