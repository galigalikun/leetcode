fn main() {
    assert_eq!(
        Solution::find_duplicate_subtrees(Some(Rc::new(RefCell::new(TreeNode {
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
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
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
        })))),
        vec![
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        ]
    );

    assert_eq!(
        Solution::find_duplicate_subtrees(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))
        })))),
        vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))]
    );

    assert_eq!(
        Solution::find_duplicate_subtrees(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))),
        vec![
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        ]
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn collect_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
        subtree_ids: &mut HashMap<(i32, i32, i32), i32>,
        subtree_counts: &mut HashMap<i32, i32>,
        next_id: &mut i32,
        duplicates: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> i32 {
        let Some(node) = root else {
            return 0;
        };

        let (val, left, right) = {
            let borrowed = node.borrow();
            (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
        };

        let left_id = Solution::collect_subtrees(left, subtree_ids, subtree_counts, next_id, duplicates);
        let right_id =
            Solution::collect_subtrees(right, subtree_ids, subtree_counts, next_id, duplicates);

        let key = (val, left_id, right_id);
        let subtree_id = *subtree_ids.entry(key).or_insert_with(|| {
            let id = *next_id;
            *next_id += 1;
            id
        });

        let count = subtree_counts.entry(subtree_id).or_insert(0);
        *count += 1;
        if *count == 2 {
            duplicates.push(Some(node));
        }

        subtree_id
    }

    fn subtree_size(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };

        let (left, right) = {
            let borrowed = node.borrow();
            (borrowed.left.clone(), borrowed.right.clone())
        };

        1 + Solution::subtree_size(&left) + Solution::subtree_size(&right)
    }

    fn root_value(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_ref().map_or(i32::MIN, |node| node.borrow().val)
    }

    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut subtree_ids: HashMap<(i32, i32, i32), i32> = HashMap::new();
        let mut subtree_counts: HashMap<i32, i32> = HashMap::new();
        let mut duplicates: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut next_id = 1;

        Solution::collect_subtrees(
            root,
            &mut subtree_ids,
            &mut subtree_counts,
            &mut next_id,
            &mut duplicates,
        );

        // Stabilize output order for deterministic testing.
        duplicates.sort_by(|a, b| {
            let size_order = Solution::subtree_size(b).cmp(&Solution::subtree_size(a));
            if size_order.is_eq() {
                Solution::root_value(b).cmp(&Solution::root_value(a))
            } else {
                size_order
            }
        });

        duplicates
    }
}
