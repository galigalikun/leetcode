fn main() {
    assert_eq!(
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        1
    );

    assert_eq!(
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
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
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                })))
            })))
        })))),
        7
    );

    assert_eq!(
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        })))),
        0
    );

    assert_eq!(
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: None,
                right: None
            })))
        })))),
        -1
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
    fn helper(ans: &mut Vec<Vec<i32>>, root: Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        if let Some(r) = root {
            if let Some(a) = ans.get_mut(depth) {
                (*a).push(r.borrow().val);
            } else {
                ans.push(vec![r.borrow().val]);
            }
            Solution::helper(ans, r.borrow().right.clone(), depth + 1);
            Solution::helper(ans, r.borrow().left.clone(), depth + 1);
        }
    }
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = Vec::new();
        Solution::helper(&mut ans, root, 0);
        return *ans.last().unwrap().last().unwrap();
    }
}
