fn main() {
    assert_eq!(
        Solution::flip_match_voyage(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            vec![2, 1]
        ),
        vec![-1]
    );

    assert_eq!(
        Solution::flip_match_voyage(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            vec![1, 3, 2]
        ),
        vec![1]
    );

    assert_eq!(
        Solution::flip_match_voyage(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            vec![1, 2, 3]
        ),
        vec![]
    );

    assert_eq!(
        Solution::flip_match_voyage(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            vec![1, 2]
        ),
        vec![]
    );

    assert_eq!(
        Solution::flip_match_voyage(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            vec![1, 2, 5, 4, 3]
        ),
        vec![2]
    );
}

struct Solution;
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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut index = 0usize;

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            voyage: &Vec<i32>,
            index: &mut usize,
            answer: &mut Vec<i32>,
        ) -> bool {
            let Some(node_rc) = node else {
                return true;
            };

            let current_val = node_rc.borrow().val;
            if *index >= voyage.len() || current_val != voyage[*index] {
                return false;
            }
            *index += 1;

            let left = node_rc.borrow().left.clone();
            let right = node_rc.borrow().right.clone();
            let expected = voyage.get(*index).copied();

            if left.is_some() && right.is_some() {
                let left_val = left.as_ref().unwrap().borrow().val;
                let right_val = right.as_ref().unwrap().borrow().val;

                if Some(left_val) == expected {
                    return dfs(left, voyage, index, answer) && dfs(right, voyage, index, answer);
                }
                if Some(right_val) == expected {
                    answer.push(current_val);
                    return dfs(right, voyage, index, answer) && dfs(left, voyage, index, answer);
                }
                return false;
            }

            if let Some(left_node) = left {
                let left_val = left_node.borrow().val;
                if Some(left_val) != expected {
                    return false;
                }
                return dfs(Some(left_node), voyage, index, answer);
            }

            if let Some(right_node) = right {
                let right_val = right_node.borrow().val;
                if Some(right_val) != expected {
                    return false;
                }
                return dfs(Some(right_node), voyage, index, answer);
            }

            true
        }

        let valid = dfs(root, &voyage, &mut index, &mut answer);
        if valid && index == voyage.len() {
            answer
        } else {
            vec![-1]
        }
    }
}
