fn main() {
    assert_eq!(Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode{
        val:0,
        left:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:Some(Rc::new(RefCell::new(TreeNode{
                val:3,
                left:None,
                right:None
            }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
                val:4,
                left:None,
                right:None
            }))),
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:2,
            left:Some(Rc::new(RefCell::new(TreeNode{
                val:3,
                left:None,
                right:None
            }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
                val:4,
                left:None,
                right:None
            }))),
        }))),
    })))), "dba");

    assert_eq!(Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode{
        val:25,
        left:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:Some(Rc::new(RefCell::new(TreeNode{
                val:1,
                left:None,
                right:None
            }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
                val:3,
                left:None,
                right:None
            }))),
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:3,
            left:Some(Rc::new(RefCell::new(TreeNode{
                val:0,
                left:None,
                right:None
            }))),
            right:Some(Rc::new(RefCell::new(TreeNode{
                val:2,
                left:None,
                right:None
            }))),
        }))),
    })))), "adz");

    assert_eq!(Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(TreeNode{
        val:2,
        left:Some(Rc::new(RefCell::new(TreeNode{
            val:2,
            left:None,
            right:Some(Rc::new(RefCell::new(TreeNode{
                val:1,
                left:Some(Rc::new(RefCell::new(TreeNode{
                val:0,
                left:None,
                right:None
            }))),
                right:None
            }))),
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:Some(Rc::new(RefCell::new(TreeNode{
                val:0,
                left:None,
                right:None
            }))),
            right:None,
        }))),
    })))), "abc");
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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = "".to_string();
        let mut stack = vec![];
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            let node = stack.last().unwrap().clone();
            if node.borrow().right.is_none() {
                let mut temp = "".to_string();
                for node in stack.iter().rev() {
                    temp.push((node.borrow().val + 'a' as i32) as u8 as char);
                }
                if result.is_empty() || temp < result {
                    result = temp;
                }
                stack.pop();
                current = None;
            } else {
                current = node.borrow().right.clone();
            }
        }
        result
    }
}
