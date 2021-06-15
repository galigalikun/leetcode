fn main() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    assert_eq!(Solution::count_smaller(vec![-1, -2]), vec![1, 0]);
}

pub struct Solution {}
// https://www.geeksforgeeks.org/count-smaller-elements-on-right-side/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub count: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, count: i32) -> Self {
        TreeNode {
            val,
            count,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn add_node(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, count: i32) -> i32 {
        if let Some(r) = root {
            if r.borrow().val < val {
                return r.borrow().count
                    + Solution::add_node(&mut r.borrow_mut().right, val, count + 1);
            } else {
                r.borrow_mut().count += 1;
                return Solution::add_node(&mut r.borrow_mut().left, val, count);
            }
        } else {
            *root = Some(Rc::new(RefCell::new(TreeNode::new(val, 0))));
            return count;
        }
    }
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        for i in (0..nums.len()).rev() {
            result[i] = Solution::add_node(&mut root, nums[i], 0);
        }
        return result;
    }
}
