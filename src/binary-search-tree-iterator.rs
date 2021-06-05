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
struct BSTIterator {
    pub data: Vec<i32>,
}

use std::cell::RefCell;
use std::rc::Rc;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn helper(
            list: &mut Vec<i32>,
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(l) = left {
                list.push(l.borrow().val);
                helper(list, l.borrow().left.clone(), l.borrow().right.clone());
            }
            if let Some(r) = right {
                list.push(r.borrow().val);
                helper(list, r.borrow().left.clone(), r.borrow().right.clone());
            }
        };
        let list = &mut vec![];
        if let Some(r) = root {
            list.push(r.borrow().val);
            helper(list, r.borrow().left.clone(), r.borrow().right.clone());
        }
        list.sort_by(|a, b| b.cmp(a));
        return BSTIterator {
            data: list.to_vec(),
        };
    }

    fn next(&mut self) -> i32 {
        if let Some(n) = self.data.pop() {
            return n;
        }
        return 0;
    }

    fn has_next(&self) -> bool {
        if self.data.len() > 0 {
            return true;
        }
        return false;
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {
    let mut obj = BSTIterator::new(Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: None,
                right: None,
            }))),
        }))),
    }))));

    assert_eq!(obj.next(), 3);
    assert_eq!(obj.next(), 7);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 9);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 15);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 20);
    assert_eq!(obj.has_next(), false);
}
