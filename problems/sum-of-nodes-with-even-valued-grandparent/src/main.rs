fn main() {
    assert_eq!(
        Solution::sum_even_grandparent(Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
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
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))),
        18
    );

    assert_eq!(
        Solution::sum_even_grandparent(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        0
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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return if let Some(r) = root {
            let mut sum = 0;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(r);
            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                if node.val % 2 == 0 {
                    if let Some(left) = &node.left {
                        let left = left.borrow();
                        if let Some(left_left) = &left.left {
                            let left_left = left_left.borrow();
                            sum += left_left.val;
                        }
                        if let Some(left_right) = &left.right {
                            let left_right = left_right.borrow();
                            sum += left_right.val;
                        }
                    }
                    if let Some(right) = &node.right {
                        let right = right.borrow();
                        if let Some(right_left) = &right.left {
                            let right_left = right_left.borrow();
                            sum += right_left.val;
                        }
                        if let Some(right_right) = &right.right {
                            let right_right = right_right.borrow();
                            sum += right_right.val;
                        }
                    }
                }
                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }
            sum
        } else {
            0
        };
    }
}
