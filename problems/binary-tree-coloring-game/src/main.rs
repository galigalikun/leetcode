fn main() {
    assert_eq!(
        Solution::btree_game_winning_move(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: None,
                            right: None
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 10,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 11,
                            left: None,
                            right: None
                        })))
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
            11,
            3
        ),
        true
    );

    assert_eq!(
        Solution::btree_game_winning_move(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            3,
            1
        ),
        false
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
    fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return 1 + Solution::count_nodes(node.borrow().left.clone())
                + Solution::count_nodes(node.borrow().right.clone());
        }
        0
    }

    fn find_node_and_count_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
        x: i32,
    ) -> Option<(i32, i32)> {
        if let Some(node) = root {
            let node_ref = node.borrow();
            if node_ref.val == x {
                let left_count = Solution::count_nodes(node_ref.left.clone());
                let right_count = Solution::count_nodes(node_ref.right.clone());
                return Some((left_count, right_count));
            }

            if let Some(result) = Solution::find_node_and_count_subtrees(node_ref.left.clone(), x) {
                return Some(result);
            }

            if let Some(result) = Solution::find_node_and_count_subtrees(node_ref.right.clone(), x) {
                return Some(result);
            }
        }
        None
    }

    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        if let Some((left_count, right_count)) = Solution::find_node_and_count_subtrees(root, x) {
            let parent_count = n - left_count - right_count - 1;
            return left_count > parent_count + right_count
                || right_count > left_count + parent_count
                || parent_count > left_count + right_count;
        }
        false
    }
}
