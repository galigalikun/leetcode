fn main() {
    assert_eq!(
        Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            9
        ),
        true
    );

    assert_eq!(
        Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            28
        ),
        false
    );
    assert_eq!(
        Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode {
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
            }))),
            3
        ),
        true
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn helper(ans: &mut HashMap<i32, i32>, k: i32, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let v = r.borrow().val;
            let d = k - v;
            if let Some(_a) = ans.get(&d) {
                return true;
            }
            if let Some(a) = ans.get_mut(&v) {
                *a += 1;
            } else {
                ans.insert(v, 1);
            }
            if Solution::helper(ans, k, r.borrow().left.clone()) {
                return true;
            }
            if Solution::helper(ans, k, r.borrow().right.clone()) {
                return true;
            }
        }
        return false;
    }
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut ans = HashMap::new();
        return Solution::helper(&mut ans, k, root);
    }
}
