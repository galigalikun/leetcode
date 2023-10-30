use std::cell::RefCell;
use std::rc::Rc;
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
struct FindElements {
    data: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if let Some(mut node) = root {
            let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![node];
            while !queue.is_empty() {
                let mut next_queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
                for node in queue {
                    let mut node = node.borrow_mut();
                    if let Some(left) = &mut node.left {
                        left.borrow_mut().val = 1; // node.val * 2 + 1;
                        next_queue.push(left.clone());
                    }
                    if let Some(right) = &mut node.right {
                        right.borrow_mut().val = 2; // node.val * 2 + 2;
                        next_queue.push(right.clone());
                    }
                }
                queue = next_queue;
            }
            println!("{:?}", queue);
            // return FindElements {
            //     data: Some(node),
            // };
        }
        return FindElements{
            data: None,
        };
    }

    fn find(&self, target: i32) -> bool {
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
        if let Some(node) = &self.data {
            queue.push(node.clone());
        }
        while !queue.is_empty() {
            let mut next_queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for node in queue {
                let node = node.borrow();
                if node.val == target {
                    return true;
                }
                if let Some(left) = &node.left {
                    next_queue.push(left.clone());
                }
                if let Some(right) = &node.right {
                    next_queue.push(right.clone());
                }
            }
            queue = next_queue;
        }
        
        return true;
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
fn main() {
    {
        let obj = FindElements::new(Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
        }))));
        assert_eq!(obj.find(1), false);
        assert_eq!(obj.find(2), true);
    }
    {
        let obj = FindElements::new(Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
        }))));
        assert_eq!(obj.find(1), true);
        assert_eq!(obj.find(3), true);
        assert_eq!(obj.find(5), false);
    }
    {
        let obj = FindElements::new(Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
                    right: None,
                }))),
                right: None,
            }))),
        }))));
        assert_eq!(obj.find(2), true);
        assert_eq!(obj.find(3), false);
        assert_eq!(obj.find(4), false);
        assert_eq!(obj.find(5), true);
    }
}
