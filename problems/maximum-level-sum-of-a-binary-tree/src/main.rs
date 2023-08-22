fn main() {
    assert_eq!(
        Solution::max_level_sum(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            })))
        })))),
        2
    );

    assert_eq!(
        Solution::max_level_sum(Some(Rc::new(RefCell::new(TreeNode {
            val: 989,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10250,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 98693,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -89388,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -32127,
                        left: None,
                        right: None
                    })))
                })))
            })))
        })))),
        2
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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = std::i32::MIN;
        let mut max_level = 0;
        let mut level = 1;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let mut sum = 0;
            let mut size = queue.len();
            while size > 0 {
                if let Some(Some(node)) = queue.pop_front() {
                    sum += node.borrow().val;
                    if let Some(left) = &node.borrow().left {
                        queue.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &node.borrow().right {
                        queue.push_back(Some(right.clone()));
                    }
                }
                size -= 1;
            }
            if sum > max {
                max = sum;
                max_level = level;
            }
            level += 1;
        }
        return max_level;
    }
}
