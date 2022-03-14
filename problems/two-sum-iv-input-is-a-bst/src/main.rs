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
    assert_eq!(Solution::find_target(Some(Rc::new(RefCell::new(TreeNode {
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
                    }))), 3), true);
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
    fn helper(ans: &mut HashMap<usize, i32>, depth:usize, root:Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if let Some(a) = ans.get_mut(&depth) {
                *a += r.borrow().val;
            } else {
                ans.insert(depth, r.borrow().val);
            }
            Solution::helper(ans, depth+1, r.borrow().left.clone());
            Solution::helper(ans, depth+1, r.borrow().right.clone());
        }
    }
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut ans = HashMap::new();
        Solution::helper(&mut ans, 1, root);
        for a in ans {
            if a.1 == k {
                return true;
            }
        }
        return false;
    }
}
