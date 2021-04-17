fn main() {
    assert_eq!(
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))
    );

    assert_eq!(Solution::sorted_array_to_bst(vec![]), None);
    assert_eq!(
        Solution::sorted_array_to_bst(vec![0]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        })))
    );
    assert_eq!(
        Solution::sorted_array_to_bst(vec![1, 3]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: None
        })))
    );
}

pub struct Solution {}
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
// https://qiita.com/ishishow/items/6227e3f1e255e8350111
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        return Some(Rc::new(RefCell::new(TreeNode{
            val: nums[nums.len()/2],
            left: Solution::sorted_array_to_bst(nums[..nums.len()/2].to_vec()),
            right: Solution::sorted_array_to_bst(nums[nums.len()/2+1..].to_vec())
        })));
    }
}
