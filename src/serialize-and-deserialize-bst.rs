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
struct Codec {
    data: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            data:None,
        }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.data = root;
        return "".to_string();
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        return self.data.clone();
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
fn main() {
    let mut obj = Codec::new();
    let data: String = obj.serialize(Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }))));
    assert_eq!(data, "");
    let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
    assert_eq!(
        ans,
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))
    );
}
