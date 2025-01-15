fn main() {
    assert_eq!(
        Solution::min_operations(vec![5, 1, 3], vec![9, 4, 2, 3, 4]),
        2
    );
    assert_eq!(
        Solution::min_operations(vec![6, 4, 8, 1, 3, 2], vec![4, 7, 6, 2, 3, 8, 6, 1]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in target.iter().enumerate() {
            map.insert(num, i);
        }
        let mut stack = Vec::new();
        for &num in arr.iter() {
            if let Some(&idx) = map.get(&num) {
                if let Err(pos) = stack.binary_search(&idx) {
                    if pos == stack.len() {
                        stack.push(idx);
                    } else {
                        stack[pos] = idx;
                    }
                }
            }
        }
        target.len() as i32 - stack.len() as i32
    }
}
