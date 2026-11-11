use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

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

fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn deepest(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            let Some(current) = node else {
                return (-1, None);
            };

            let left = deepest(current.borrow().left.as_ref());
            let right = deepest(current.borrow().right.as_ref());

            match left.0.cmp(&right.0) {
                std::cmp::Ordering::Greater => (left.0 + 1, left.1),
                std::cmp::Ordering::Less => (right.0 + 1, right.1),
                std::cmp::Ordering::Equal => (left.0 + 1, Some(Rc::clone(current))),
            }
        }

        deepest(root.as_ref()).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(
            3,
            node(
                5,
                node(6, None, None),
                node(
                    2,
                    node(7, None, None),
                    node(4, None, None),
                ),
            ),
            node(
                1,
                node(0, None, None),
                node(8, None, None),
            ),
        );

        let expected = node(
            2,
            node(7, None, None),
            node(4, None, None),
        );

        assert_eq!(Solution::lca_deepest_leaves(root), expected);
    }

    #[test]
    fn single_node() {
        let root = node(1, None, None);
        let expected = node(1, None, None);

        assert_eq!(Solution::lca_deepest_leaves(root), expected);
    }

    #[test]
    fn deepest_leaf_is_only_leaf_in_its_subtree() {
        let root = node(
            0,
            node(
                1,
                None,
                node(2, None, None),
            ),
            node(3, None, None),
        );

        let expected = node(2, None, None);

        assert_eq!(Solution::lca_deepest_leaves(root), expected);
    }
}
