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
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> String {
            match node {
                None => String::new(),
                Some(current) => {
                    let ch = (b'a' + current.borrow().val as u8) as char;
                    let left = dfs(&current.borrow().left);
                    let right = dfs(&current.borrow().right);

                    if left.is_empty() && right.is_empty() {
                        return ch.to_string();
                    }

                    let mut best = left;
                    if !right.is_empty() && (best.is_empty() || right < best) {
                        best = right;
                    }

                    if best.is_empty() {
                        ch.to_string()
                    } else {
                        format!("{}{}", best, ch)
                    }
                }
            }
        }

        dfs(&root)
    }
}
