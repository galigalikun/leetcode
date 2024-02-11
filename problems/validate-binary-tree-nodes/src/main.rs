fn main() {
    assert_eq!(
        Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
        true
    );
    assert_eq!(
        Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
        false
    );
    assert_eq!(
        Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]),
        false
    );
    assert_eq!(
        Solution::validate_binary_tree_nodes(4, vec![1, 0, 3, -1], vec![-1, -1, -1, -1]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut parent = vec![0; n as usize];
        for i in 0..n {
            if left_child[i as usize] != -1 {
                if parent[left_child[i as usize] as usize] != 0 {
                    return false;
                }
                parent[left_child[i as usize] as usize] = i + 1;
            }
            if right_child[i as usize] != -1 {
                if parent[right_child[i as usize] as usize] != 0 {
                    return false;
                }
                parent[right_child[i as usize] as usize] = i + 1;
            }
        }
        let mut root = 0;
        for i in 0..n {
            if parent[i as usize] == 0 {
                root += 1;
            }
        }
        if root != 1 {
            return false;
        }
        return true;
    }
}
