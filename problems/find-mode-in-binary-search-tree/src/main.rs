fn main() {
    assert_eq!(
        Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))),
        vec![2]
    );

    assert_eq!(
        Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        })))),
        vec![0]
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
    fn helper(map: &mut HashMap<i32, i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if let Some(m) = map.get_mut(&r.borrow().val) {
                *m += 1;
            } else {
                map.insert(r.borrow().val, 1);
            }
            Solution::helper(map, r.borrow().left.clone());
            Solution::helper(map, r.borrow().right.clone());
        }
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        Solution::helper(&mut map, root);
        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_by(|x, y| y.1.cmp(&x.1));
        let mut result = vec![];
        let mut prev: Option<i32> = None;
        for (n, c) in v {
            if prev != None && Some(c) != prev {
                break;
            }
            result.push(n);
            prev = Some(c);
        }
        return result;
    }
}
