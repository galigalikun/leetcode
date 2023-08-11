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
    fn find_x_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        x: i32,
        x_node: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root {
            if node.borrow().val == x {
                *x_node = Some(node.clone());
            } else {
                Solution::find_x_node(node.borrow().left.clone(), x, x_node);
                Solution::find_x_node(node.borrow().right.clone(), x, x_node);
            }
        }
    }
    fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>, left: &mut i32, right: &mut i32) -> i32 {
        if let Some(node) = root {
            let left_count = Solution::count_nodes(node.borrow().left.clone(), left, right);
            let right_count = Solution::count_nodes(node.borrow().right.clone(), left, right);
            if left_count > right_count {
                *left += left_count + 1;
            } else {
                *right += right_count + 1;
            }
            return left_count + right_count + 1;
        }
        return 0;
    }
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut left = 0;
        let mut right = 0;
        let mut x_node = None;
        Solution::find_x_node(root.clone(), x, &mut x_node);
        Solution::count_nodes(x_node.clone(), &mut left, &mut right);
        let parent = n - left - right - 1;
        return parent > left + right || left > parent + right || right > parent + left;
    }
}
