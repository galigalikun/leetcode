fn main() {
    println!("Hello, world!");
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
            let Some(node_rc) = node else {
                return 0;
            };

            let (val, left, right) = {
                let node = node_rc.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };

            let left_balance = dfs(left, moves);
            let right_balance = dfs(right, moves);

            *moves += left_balance.abs() + right_balance.abs();
            val + left_balance + right_balance - 1
        }

        let mut moves = 0;
        dfs(root, &mut moves);
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn example_1() {
        let root = node(3, node(0, None, None), node(0, None, None));
        assert_eq!(Solution::distribute_coins(root), 2);
    }

    #[test]
    fn example_2() {
        let root = node(0, node(3, None, None), node(0, None, None));
        assert_eq!(Solution::distribute_coins(root), 3);
    }

    #[test]
    fn example_3() {
        let root = node(1, node(0, None, None), node(2, None, None));
        assert_eq!(Solution::distribute_coins(root), 2);
    }
}
