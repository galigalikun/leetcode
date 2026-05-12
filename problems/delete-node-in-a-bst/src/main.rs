fn main() {
    assert_eq!(
        Solution::delete_node(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            3
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                })))
            })))
        })))
    );

    assert_eq!(
        Solution::delete_node(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            0
        ),
        None
    );

    assert_eq!(Solution::delete_node(None, 0), None);
}

struct Solution {}
// https://www.techiedelight.com/deletion-from-bst/
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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut curr = root.clone();
        let mut parent: Option<Rc<RefCell<TreeNode>>> = None;
        loop {
            if let Some(c) = curr.clone() {
                let v = c.borrow().val;
                if v == key {
                    break;
                }
                parent = curr;
                if key < v {
                    curr = c.borrow().left.clone();
                } else {
                    curr = c.borrow().right.clone();
                }
            } else {
                break;
            }
        }
        if let Some(c) = curr {
            match (c.borrow().left.clone(), c.borrow().right.clone()) {
                ((Some(l), Some(r))) => {}
                (None, None) => {}
                _ => {
                    // todo
                }
            }
        } else {
            return root;
        }
        return None;
    }
}

struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn preorder(node: Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<String>) {
            if let Some(current) = node {
                let (value, left, right) = {
                    let borrowed = current.borrow();
                    (
                        borrowed.val,
                        borrowed.left.clone(),
                        borrowed.right.clone(),
                    )
                };
                out.push(value.to_string());
                preorder(left, out);
                preorder(right, out);
            }
        }

        let mut values = Vec::new();
        preorder(root, &mut values);
        values.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let values: Vec<i32> = data
            .split(',')
            .map(|part| {
                part.parse::<i32>()
                    .expect("serialization input contains an invalid i32")
            })
            .collect();

        fn build(
            values: &[i32],
            index: &mut usize,
            lower: i64,
            upper: i64,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if *index >= values.len() {
                return None;
            }

            let value = values[*index] as i64;
            if value <= lower || value >= upper {
                return None;
            }

            *index += 1;

            let left = build(values, index, lower, value);
            let right = build(values, index, value, upper);

            Some(Rc::new(RefCell::new(TreeNode {
                val: value as i32,
                left,
                right,
            })))
        }

        let mut index = 0;
        build(&values, &mut index, i64::MIN, i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_empty_tree() {
        let codec = Codec::new();
        let serialized = codec.serialize(None);
        assert_eq!(serialized, "");
        assert_eq!(codec.deserialize(serialized), None);
    }

    #[test]
    fn serialize_deserialize_bst_round_trip() {
        let codec = Codec::new();

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
            }))),
        })));

        let serialized = codec.serialize(root.clone());
        assert_eq!(serialized, "8,3,1,6,4,7,10,14");

        let deserialized = codec.deserialize(serialized);
        assert_eq!(deserialized, root);
    }
}
