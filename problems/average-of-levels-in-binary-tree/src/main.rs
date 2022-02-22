fn main() {
    assert_eq!(
        Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
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
        vec![3.00000, 14.50000, 11.00000]
    );

    assert_eq!(
        Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: None,
                right: None
            })))
        })))),
        vec![3.00000, 14.50000, 11.00000]
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
    fn helper(ans: &mut Vec<Vec<f64>>, depth: usize, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if ans.len() <= depth {
                ans.push(vec![r.borrow().val as f64]);
            } else {
                ans[depth].push(r.borrow().val as f64);
            }
            Solution::helper(ans, depth + 1, r.borrow().left.clone());
            Solution::helper(ans, depth + 1, r.borrow().right.clone());
        }
    }
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut ans: Vec<Vec<f64>> = vec![];
        Solution::helper(&mut ans, 0, root);
        return ans
            .iter()
            .map(|x| x.iter().fold(0_f64, |a, b| a + b) / x.len() as f64)
            .collect::<Vec<_>>();
    }
}
