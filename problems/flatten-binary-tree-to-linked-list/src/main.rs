fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    })));

    Solution::flatten(&mut root);
    assert_eq!(
        root,
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: None,
                                right: None
                            })))
                        })))
                    })))
                })))
            })))
        })))
    );

    root = None;
    Solution::flatten(&mut root);
    assert_eq!(root, None);

    root = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: None,
        right: None,
    })));
    Solution::flatten(&mut root);
    assert_eq!(
        root,
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
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
    fn helper(list: &mut Vec<i32>, root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            list.push(r.borrow().val);
            Solution::helper(list, &mut r.borrow().left.clone());
            Solution::helper(list, &mut r.borrow().right.clone());
        }
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut list = vec![];
        Solution::helper(&mut list, root);

        let mut result: Option<Rc<RefCell<TreeNode>>> = None;
        for i in list.into_iter().rev() {
            result = Some(Rc::new(RefCell::new(TreeNode {
                val: i,
                left: None,
                right: result,
            })));
        }

        *root = result;
    }
}
