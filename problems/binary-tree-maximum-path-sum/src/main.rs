fn main() {
    assert_eq!(
        Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode {
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
        })))),
        6
    );

    assert_eq!(
        Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode {
            val: -10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                })))
            })))
        })))),
        42
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/find-maximum-path-sum-in-a-binary-tree/
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
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(r) = root {
            let data = r.borrow().val;
            let l = Solution::helper(r.borrow().left.clone(), result);
            let r = Solution::helper(r.borrow().right.clone(), result);
            let max_single = std::cmp::max(std::cmp::max(l, r) + data, data);
            let max_top = std::cmp::max(max_single, l + r + data);
            if &max_top > result {
                *result = max_top;
            }

            return max_single;
        }
        return 0;
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = std::i32::MIN;
        Solution::helper(root, &mut result);
        return result;
    }
}
