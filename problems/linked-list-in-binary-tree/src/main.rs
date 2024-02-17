fn main() {
    assert_eq!(
        Solution::is_sub_path(
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        }))),
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 2,
                                left: None,
                                right: None
                            })))
                        })))
                    }))),
                    right: None
                })))
            }))),
        ),
        true
    );

    assert_eq!(
        Solution::is_sub_path(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 6, next: None }))
                    }))
                }))
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        }))),
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 2,
                                left: None,
                                right: None
                            })))
                        })))
                    }))),
                    right: None
                })))
            })))
        ),
        true
    );

    assert_eq!(
        Solution::is_sub_path(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode { val: 8, next: None }))
                        }))
                    }))
                }))
            })),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        }))),
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 2,
                                left: None,
                                right: None
                            })))
                        })))
                    }))),
                    right: None
                })))
            })))
        ),
        false
    );
}

struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let head = head;
        let root = root;
        let mut queue = vec![(root, head)];
        while !queue.is_empty() {
            let (root, head) = queue.pop().unwrap();
            if let Some(root) = root {
                let root = root.borrow();
                if head.is_none() {
                    return true;
                }
                let head = head.unwrap();
                if root.val == head.val {
                    queue.push((root.left.clone(), head.next.clone()));
                    queue.push((root.right.clone(), head.next.clone()));
                }
            }
        }

        return false;
    }
}
