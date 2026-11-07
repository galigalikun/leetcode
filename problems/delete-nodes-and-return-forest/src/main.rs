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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    fn del_nodes_sub(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        is_root: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node_rc = root?;

        let mut node = node_rc.borrow_mut();
        let deleted = to_delete.contains(&node.val);

        if is_root && !deleted {
            result.push(Some(node_rc.clone()));
        }

        let left = node.left.take();
        let right = node.right.take();
        node.left = Solution::del_nodes_sub(left, to_delete, result, deleted);
        node.right = Solution::del_nodes_sub(right, to_delete, result, deleted);

        drop(node);

        if deleted {
            None
        } else {
            Some(node_rc)
        }
    }
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];
        let delete_set: HashSet<i32> = to_delete.into_iter().collect();
        Solution::del_nodes_sub(root, &delete_set, &mut result, true);
        result
    }
}
