fn main() {
    assert_eq!(
        Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                })))
            })))
        })))),
        4
    );

    assert_eq!(
        Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                })))
            }))),
        })))),
        7
    );

    assert_eq!(
        Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })))
        })))),
        2
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
    fn helper(
        result: &mut Vec<Vec<String>>,
        depth: usize,
        idx: usize,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(r) = root {
            if result.len() == depth {
                result.push(vec![String::new(); 2_usize.pow(depth as u32)]);
            }
            result[depth][idx] = format!("{}", r.borrow().val);
            Solution::helper(result, depth + 1, idx * 2, r.borrow().left.clone());
            Solution::helper(result, depth + 1, idx * 2 + 1, r.borrow().right.clone());
        }
    }
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = vec![];
        Solution::helper(&mut result, 0, 0, root);
        let mut ans = 0;
        for a in result {
            let mut start = std::usize::MAX;
            let mut end = 0;
            for i in 0..a.len() {
                if !a[i].is_empty() {
                    start = std::cmp::min(i, start);
                    end = std::cmp::max(i, end);
                }
            }
            ans = std::cmp::max(ans, 1 + end - start);
        }
        return ans as i32;
    }
}
