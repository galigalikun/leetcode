fn main() {
    assert_eq!(
        Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
            }))),
        })))),
        "dba"
    );

    assert_eq!(
        Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 25,
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
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
            }))),
        })))),
        "adz"
    );

    assert_eq!(
        Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: None,
            }))),
        })))),
        "abc"
    );

    assert_eq!(
        Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })))),
        "bae"
    );

    assert_eq!(
        Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 25,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
            }))),
            right: None,
        })))),
        "ababz"
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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            match node {
                None => vec![],
                Some(current) => {
                    let ch = (b'a' + current.borrow().val as u8) as char;
                    let left = dfs(&current.borrow().left);
                    let right = dfs(&current.borrow().right);
                    let mut paths = Vec::new();

                    for candidate in [left, right] {
                        for path in candidate {
                            paths.push(format!("{}{}", path, ch));
                        }
                    }

                    if paths.is_empty() {
                        vec![ch.to_string()]
                    } else {
                        paths
                    }
                }
            }
        }

        let mut paths = dfs(&root);
        paths.sort();
        paths[0].clone()
    }
}
