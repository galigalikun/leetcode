fn main() {
    assert_eq!(
        Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            })))
        })))),
        5
    );

    assert_eq!(
        Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
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
        })))),
        -1
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn helper(lst: &mut HashMap<i32, i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if let Some(v) = lst.get(&r.borrow().val) {
                if *v == 1 {
                    lst.insert(r.borrow().val, 2);
                }
            } else {
                lst.insert(r.borrow().val, 1);
            }
            Solution::helper(lst, r.borrow().left.clone());
            Solution::helper(lst, r.borrow().right.clone());
        }
    }
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut dic: HashMap<i32, i32> = HashMap::new();
        Solution::helper(&mut dic, root);
        if dic.len() > 1 {
            let mut v = dic.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
            v.sort();
            return *v[1].0;
        }
        return -1;
    }
}
