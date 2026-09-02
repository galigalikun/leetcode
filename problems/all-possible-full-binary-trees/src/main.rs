fn main() {
    assert_eq!(
        Solution::all_possible_fbt(7),
        vec![
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        })))
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                })))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                })))
            })))
        ]
    );

    assert_eq!(
        Solution::all_possible_fbt(3),
        vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            })))
        })))]
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut memo: HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
        Self::build_trees(n, &mut memo)
    }

    fn build_trees(
        n: i32,
        memo: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 || n <= 0 {
            return vec![];
        }
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        if let Some(cached) = memo.get(&n) {
            return cached.clone();
        }

        let mut trees = Vec::new();
        for left_nodes in (1..n).step_by(2) {
            let right_nodes = n - 1 - left_nodes;
            let left_trees = Self::build_trees(left_nodes, memo);
            let right_trees = Self::build_trees(right_nodes, memo);

            for left_tree in &left_trees {
                for right_tree in &right_trees {
                    trees.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left_tree.clone(),
                        right: right_tree.clone(),
                    }))));
                }
            }
        }

        memo.insert(n, trees.clone());
        trees
    }
}
