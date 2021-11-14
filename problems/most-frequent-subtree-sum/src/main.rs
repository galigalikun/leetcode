fn main() {
    assert_eq!(
        Solution::find_frequent_tree_sum(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: None
            })))
        })))),
        vec![2, -3, 4]
    );

    assert_eq!(
        Solution::find_frequent_tree_sum(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -5,
                left: None,
                right: None
            })))
        })))),
        vec![2]
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
    fn helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, Vec<i32>>,
        frequency: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(r) = root {
            let left = Solution::helper(r.borrow().left.clone(), map, frequency);
            let right = Solution::helper(r.borrow().right.clone(), map, frequency);
            let total = left + right + r.borrow().val;

            if let Some(f) = frequency.get_mut(&total) {
                *f += 1;
                if let Some(m) = map.get_mut(f) {
                    (*m).push(total);
                } else {
                    map.insert(*f, vec![total]);
                }
            } else {
                if let Some(m) = map.get_mut(&1) {
                    (*m).push(total);
                } else {
                    map.insert(1, vec![total]);
                }
                frequency.insert(total, 1);
            }

            return total;
        } else {
            return 0;
        }
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root == None {
            return vec![];
        }
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut frequency = HashMap::new();
        Solution::helper(root, &mut map, &mut frequency);
        let mut max = std::i32::MIN;
        for (key, _val) in map.iter() {
            max = std::cmp::max(max, *key);
        }

        return map.get(&max).unwrap().to_vec();
    }
}
