fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
        right: None,
    })));
    Solution::recover_tree(&mut root);
    assert_eq!(
        root,
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            }))),
            right: None
        })))
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
use std::rc::Rc;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first = None;
        let mut second = None;
        let mut previous = None;

        Self::find_swapped_nodes(root, &mut previous, &mut first, &mut second);

        if let (Some(first_node), Some(second_node)) = (first, second) {
            let first_value = first_node.borrow().val;
            let second_value = second_node.borrow().val;

            first_node.borrow_mut().val = second_value;
            second_node.borrow_mut().val = first_value;
        }
    }

    fn find_swapped_nodes(
        node: &Option<Rc<RefCell<TreeNode>>>,
        previous: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        let Some(current) = node.clone() else {
            return;
        };

        let left = current.borrow().left.clone();
        Self::find_swapped_nodes(&left, previous, first, second);

        if let Some(prev_node) = previous {
            let prev_value = prev_node.borrow().val;
            let current_value = current.borrow().val;

            if prev_value > current_value {
                if first.is_none() {
                    *first = Some(prev_node.clone());
                }
                *second = Some(current.clone());
            }
        }

        *previous = Some(current.clone());

        let right = current.borrow().right.clone();
        Self::find_swapped_nodes(&right, previous, first, second);
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
    fn recovers_adjacent_swapped_nodes() {
        let mut root = node(1, node(3, None, node(2, None, None)), None);

        Solution::recover_tree(&mut root);

        assert_eq!(root, node(3, node(1, None, node(2, None, None)), None));
    }

    #[test]
    fn recovers_non_adjacent_swapped_nodes() {
        let mut root = node(
            3,
            node(1, None, None),
            node(4, node(2, None, None), None),
        );

        Solution::recover_tree(&mut root);

        assert_eq!(
            root,
            node(2, node(1, None, None), node(4, node(3, None, None), None))
        );
    }
}
