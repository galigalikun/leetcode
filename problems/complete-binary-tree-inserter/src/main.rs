use std::{cell::RefCell, rc::Rc};

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
struct CBTInserter {
    root: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut v = vec![];
        CBTInserter::dp(&mut v, root);
        CBTInserter { root: v }
    }

    fn dp(v: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            v.push(r.borrow().val);
            CBTInserter::dp(v, r.borrow().left.clone());
            CBTInserter::dp(v, r.borrow().right.clone());
        }
    }

    fn insert(&mut self, val: i32) -> i32 {
        self.root.push(val);
        return (self.root.len() as f64).sqrt() as i32;
    }

    fn get_root(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        let mut a = None;
        if let Some(v) = self.root.last() {
            a = Some(Rc::new(RefCell::new(TreeNode {
                val: *v,
                left: None,
                right: None,
            })))
        }
        for i in (1..self.root.len()).step_by(2).rev() {
            a = Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: self.root[i],
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: self.root[i-1],
                    left: None,
                    right: None,
                }))),
            })));
        }
        return a;
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
fn main() {
    let mut obj = CBTInserter::new(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: None,
    }))));
    assert_eq!(1, obj.insert(3));
    assert_eq!(2, obj.insert(4));
    assert_eq!(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })))
        }))),
        obj.get_root()
    );
}
