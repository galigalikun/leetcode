fn main() {
    assert_eq!(
        Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            })))
        })))),
        2
    );
    assert_eq!(
        Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))
        })))),
        1
    );
    assert_eq!(
        Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(TreeNode::new(9))))),
        1
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
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, path: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            path[node.val as usize] += 1;
            if node.left.is_none() && node.right.is_none() {
                let mut odd = 0;
                for i in 1..10 {
                    if path[i] % 2 == 1 {
                        odd += 1;
                    }
                }
                if odd <= 1 {
                    *count += 1;
                }
            } else {
                Self::dfs(&node.left, count, path);
                Self::dfs(&node.right, count, path);
            }
            path[node.val as usize] -= 1;
        }
    }
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        let mut path = vec![0; 10];
        Self::dfs(&root, &mut count, &mut path);
        return count;
    }
}
