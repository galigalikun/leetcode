fn main() {
    assert_eq!(
        Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None
                })))
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
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
        Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))
    );

    assert_eq!(
        Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None
        })))
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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut max_depth = 0;
        let mut max_depth_node = None;
        fn find_max_depth(
            node: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: &mut i32,
            max_depth_node: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                if depth > *max_depth {
                    *max_depth = depth;
                    *max_depth_node = Some(n.clone());
                }
                find_max_depth(
                    n.borrow().left.clone(),
                    depth + 1,
                    max_depth,
                    max_depth_node,
                );
                find_max_depth(
                    n.borrow().right.clone(),
                    depth + 1,
                    max_depth,
                    max_depth_node,
                );
            }
        }
        find_max_depth(root.clone(), 0, &mut max_depth, &mut max_depth_node);
        let mut result = None;
        fn find_lca(
            node: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: i32,
            max_depth_node: Option<Rc<RefCell<TreeNode>>>,
            result: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> i32 {
            if let Some(n) = node {
                let left_depth = find_lca(
                    n.borrow().left.clone(),
                    depth + 1,
                    max_depth,
                    max_depth_node.clone(),
                    result,
                );
                let right_depth = find_lca(
                    n.borrow().right.clone(),
                    depth + 1,
                    max_depth,
                    max_depth_node.clone(),
                    result,
                );
                if left_depth == max_depth && right_depth == max_depth {
                    *result = Some(n.clone());
                }
                return std::cmp::max(left_depth, right_depth);
            }
            return depth;
        }
        find_lca(
            root.clone(),
            0,
            max_depth,
            max_depth_node.clone(),
            &mut result,
        );
        return result;
    }
}
