fn main() {
    assert_eq!(
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
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
                left: None,
                right: None
            })))
        })))),
        "1(2(4))(3)"
    );
    assert_eq!(
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        "1(2()(4))(3)"
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
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(r) = root {
            return format!(
                "({}{}{})",
                r.borrow().val,
                Solution::helper(r.borrow().left.clone(), r.borrow().right.clone()),
                Solution::helper(r.borrow().right.clone(), None)
            );
        } else if right != None {
            return "()".to_string();
        }
        return "".to_string();
    }
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let ans = Solution::helper(root, None);
        if ans.len() > 0 {
            return ans[1..ans.len() - 1].to_string();
        }
        return ans;
    }
}
