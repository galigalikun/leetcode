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
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
struct Helper {
    prev: Option<Rc<RefCell<TreeNode>>>,
    node1: Option<Rc<RefCell<TreeNode>>>,
    node2: Option<Rc<RefCell<TreeNode>>>,
}
impl Helper {
    pub fn new() -> Self {
        Helper {
            prev: None,
            node1: None,
            node2: None,
        }
    }

    pub fn help(&mut self, root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            self.help(&mut r.borrow_mut().left);
            if let Some(p) = &self.prev {
                if p.borrow().val > r.borrow().val {
                    // if self.node1 == None {
                    //     self.node1 = self.prev;
                    // }
                    // self.node2 = Some(Rc::new(RefCell::new(&*r.borrow())));
                    // self.node2 = Some(Rc::new(RefCell::new(*node)));
                    // self.node2 = Some(Rc::new(RefCell::new(r.borrow())));
                }
            }
            // self.prev = *root;
            self.help(&mut r.borrow_mut().right);
        }
    }
}
impl Solution {
    // fn helper(
    //     root: &mut Option<Rc<RefCell<TreeNode>>>,
    //     node1: &Option<Rc<RefCell<TreeNode>>>,
    //     node2: &Option<Rc<RefCell<TreeNode>>>,
    //     prev: &Option<Rc<RefCell<TreeNode>>>,
    // ) {
    //     if let Some(r) = root {
    //         Solution::helper(&mut r.borrow_mut().left, node1, node2, prev);
    //     //     Solution::helper(&mut r.borrow_mut().left, prev, node1, node2);

    //         if let Some(p) = prev {
    //             if p.borrow().val > r.borrow().val {
    //     //             if let Some(_n1) = node1 {
    //     //             } else {
    //     //                 *node1 = Some(p);
    //     //             }
    //                 let node2 = r;

    //             }
    //         }
    //         // Solution::helper(&mut r.borrow_mut().right, node1, node2, r.borrow());
    //     //     Solution::helper(&mut r.borrow_mut().right, Some(r), node1, node2);
    //     }
    // }
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // let helper = Helper::new();
        // Solution::helper(root, &None, &None, &None);
        let mut prev: Option<TreeNode> = None;

        struct Dfs<'s> {
            f: &'s dyn Fn(&Dfs, &mut Option<Rc<RefCell<TreeNode>>>),
        }
        let dfs = Dfs {
            f: &|dfs, root| {
                if let Some(r) = root {
                    let prev = Some(r.borrow());
                }
            },
        };

        (dfs.f)(&dfs, root);

        // if result.len() > 0 {
        //     if result.len() == 2 {
        //     } else if result.len() == 3 {
        //     } else if result.len() == 4 {
        //     }
        // }
    }
}
