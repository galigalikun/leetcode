fn main() {
    assert_eq!(
        Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: None
            })))
        })))
    );

    assert_eq!(
        Solution::construct_maximum_binary_tree(vec![3, 2, 1]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut idx: Option<usize> = None;
        let mut max_num: Option<i32> = None;
        for i in 0..nums.len() {
            if max_num < Some(nums[i]) {
                idx = Some(i);
                max_num = Some(nums[i]);
            }
        }
        return if let Some(i) = idx {
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[i],
                left: Solution::construct_maximum_binary_tree(nums[0..i].to_vec()),
                right: Solution::construct_maximum_binary_tree(nums[i + 1..].to_vec()),
            })))
        } else {
            None
        };
    }
}
