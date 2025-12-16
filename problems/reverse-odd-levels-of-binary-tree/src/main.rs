fn main() {
    assert_eq!(
        Solution::reverse_odd_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(21)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(34)))),
            }))),
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(21)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(34)))),
            }))),
        })))
    );
    assert_eq!(
        Solution::reverse_odd_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
        })))
    );
    assert_eq!(
        Solution::reverse_odd_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })))),
        None
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
pub fn reverse_odd_levels(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes = vec![];
    Self::dfs(&root, 0, &mut nodes);
    for i in (1..nodes.len()).step_by(2) {
        let level = &mut nodes[i];
        let n = level.len();
        for j in 0..n / 2 {
            let left = level[j].borrow().val;
            let right = level[n - 1 - j].borrow().val;
            level[j].borrow_mut().val = right;
            level[n - 1 - j].borrow_mut().val = left;
        }
    }
    root
}
fn dfs(
    node: &Option<Rc<RefCell<TreeNode>>>,
    level: usize,
    nodes: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>,
) {
    if let Some(n) = node {
        if nodes.len() == level {
            nodes.push(vec![]);
        }
        nodes[level].push(Rc::clone(n));
        Self::dfs(&n.borrow().left, level + 1, nodes);
        Self::dfs(&n.borrow().right, level + 1, nodes);
    }
}
}
