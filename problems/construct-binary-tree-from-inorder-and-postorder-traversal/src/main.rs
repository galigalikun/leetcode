fn main() {
    assert_eq!(
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
        Some(Rc::new(RefCell::new(TreeNode {
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
        })))
    );

    assert_eq!(
        Solution::build_tree(vec![-1], vec![-1]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: None
        })))
    );
}

pub struct Solution {}
// https://propyon.hateblo.jp/entry/2020/04/16/135222
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
    fn helper(
        low: i32,
        high: i32,
        postorder: &mut Vec<i32>,
        inorder_dic: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if low > high {
            return None;
        }
        if let Some(x) = postorder.pop() {
            let mid = inorder_dic[&x];
            return Some(Rc::new(RefCell::new(TreeNode {
                val: x,
                right: Solution::helper(mid as i32 + 1, high, postorder, inorder_dic),
                left: Solution::helper(low, mid as i32 - 1, postorder, inorder_dic),
            })));
        }

        return None;
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_dic: HashMap<i32, usize> = HashMap::new();

        for i in 0..inorder.len() {
            inorder_dic.insert(inorder[i], i);
        }

        return Solution::helper(
            0,
            inorder.len() as i32 - 1,
            &mut postorder.clone(),
            &inorder_dic,
        );
    }
}
