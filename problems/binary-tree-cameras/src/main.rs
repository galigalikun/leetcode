fn main() {
    assert_eq!(
        Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        })))),
        1
    );

    assert_eq!(
        Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        })))),
        2
    );
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cameras = 0;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cameras: &mut i32) -> i32 {
            if node.is_none() {
                return 0;
            }

            let node_ref = node.as_ref().unwrap();
            let left = dfs(&node_ref.borrow().left, cameras);
            let right = dfs(&node_ref.borrow().right, cameras);

            if left == 2 || right == 2 {
                *cameras += 1;
                return 1;
            }

            if left == 1 || right == 1 {
                return 0;
            }

            2
        }

        let root_state = dfs(&root, &mut cameras);
        if root_state == 2 {
            cameras += 1;
        }

        cameras
    }
}
