fn main() {
    assert_eq!(
        Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None
                })))
            })))
        })))),
        [1, 3, 9]
    );
    assert_eq!(
        Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode {
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
        vec![1, 3]
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
    fn helper(ans: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        if let Some(r) = root {
            if ans.len() > depth {
                ans[depth] = std::cmp::max(ans[depth], r.borrow().val);
            } else {
                ans.push(r.borrow().val);
            }
            Solution::helper(ans, r.borrow().left.clone(), depth + 1);
            Solution::helper(ans, r.borrow().right.clone(), depth + 1);
        }
    }
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        Solution::helper(&mut ans, root, 0);
        return ans;
    }
}
