use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

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
    root: Option<Rc<RefCell<TreeNode>>>,
    candidates: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut candidates = VecDeque::new();
        let mut queue = VecDeque::new();

        if let Some(node) = root.clone() {
            queue.push_back(node);
        }

        while let Some(node) = queue.pop_front() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if left.is_none() || right.is_none() {
                candidates.push_back(node.clone());
            }

            if let Some(left_node) = left {
                queue.push_back(left_node);
            }

            if let Some(right_node) = right {
                queue.push_back(right_node);
            }
        }

        Self { root, candidates }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
        let parent = self
            .candidates
            .front()
            .cloned()
            .expect("root must exist in CBTInserter");
        let parent_val = parent.borrow().val;

        let has_left = parent.borrow().left.is_some();
        if !has_left {
            parent.borrow_mut().left = Some(new_node.clone());
        } else {
            parent.borrow_mut().right = Some(new_node.clone());
            self.candidates.pop_front();
        }

        self.candidates.push_back(new_node);
        parent_val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}
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
