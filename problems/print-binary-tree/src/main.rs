fn main() {
    assert_eq!(
        Solution::print_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: None
        })))),
        vec![["", "1", ""], ["2", "", ""]]
    );

    assert_eq!(
        Solution::print_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        vec![
            ["", "", "", "1", "", "", ""],
            ["", "2", "", "", "", "3", ""],
            ["", "", "4", "", "", "", ""]
        ]
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
    fn helper(result: &mut Vec<Vec<String>>, depth:usize, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            if result.len() == depth {
                result.push(vec![format!("{}", r.borrow().val)]);
            } else {
                result[depth].push(format!("{}", r.borrow().val));
            }
            Solution::helper(result, depth+1, r.borrow().left.clone());
            Solution::helper(result, depth+1, r.borrow().right.clone());
        }
    }
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        // 1 = 2^0 => n = 0
        // 3 = 2^1 + 2^0 => n = 1
        // 7 = 2^2 + 2^1 + 2^0 => n = 2
        // 15 = 2^3 + 2^2 + 2^1 + 2^0
        // 31 = 2^4 + 2^3 + 2^2 + 2^1 + 2^0
        // f(n) = 2*f(n-1) + 1
        // f(n) = 2^(n+1) - 1
        let mut result = vec![];
        Solution::helper(&mut result, 0, root);
        let size = 2_usize.pow(result.len() as u32) - 1;
        let mut ans = vec![];
        for i in 0..result.len() {
            ans.push(vec![String::new(); size]);
            for j in 0..result[i].len() {
                ans[i][j] = result[i][j].clone();
            }
        }
        println!("debug {:?} {} {}", result, result.len(), size);
        return ans;
    }
}
