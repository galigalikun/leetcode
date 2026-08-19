fn main() {
    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }

    let root = Some(node(
        3,
        Some(node(
            5,
            Some(node(6, None, None)),
            Some(node(2, Some(node(7, None, None)), Some(node(4, None, None)))),
        )),
        Some(node(1, Some(node(0, None, None)), Some(node(8, None, None)))),
    ));

    let mut got = Solution::distance_k(root, Some(node(5, None, None)), 2);
    let mut expected = vec![7, 4, 1];
    got.sort_unstable();
    expected.sort_unstable();
    assert_eq!(got, expected);

    assert_eq!(
        Solution::distance_k(Some(node(1, None, None)), Some(node(1, None, None)), 3),
        vec![]
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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        fn build_graph(
            node: Option<Rc<RefCell<TreeNode>>>,
            parent: Option<i32>,
            graph: &mut HashMap<i32, Vec<i32>>,
        ) {
            let Some(node) = node else {
                return;
            };

            let (val, left, right) = {
                let borrowed = node.borrow();
                (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
            };

            graph.entry(val).or_default();
            if let Some(parent_val) = parent {
                graph.entry(val).or_default().push(parent_val);
                graph.entry(parent_val).or_default().push(val);
            }

            build_graph(left, Some(val), graph);
            build_graph(right, Some(val), graph);
        }

        let Some(target) = target else {
            return vec![];
        };
        let target_val = target.borrow().val;

        let mut graph = HashMap::new();
        build_graph(root, None, &mut graph);

        let mut queue = VecDeque::from([(target_val, 0)]);
        let mut visited = HashSet::from([target_val]);
        let mut result = vec![];

        while let Some((current, dist)) = queue.pop_front() {
            if dist == k {
                result.push(current);
                continue;
            }
            if dist > k {
                continue;
            }

            if let Some(neighbors) = graph.get(&current) {
                for &next in neighbors {
                    if visited.insert(next) {
                        queue.push_back((next, dist + 1));
                    }
                }
            }
        }

        result
    }
}
