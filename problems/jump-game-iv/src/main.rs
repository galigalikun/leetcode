fn main() {
    assert_eq!(
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    assert_eq!(Solution::min_jumps(vec![7]), 0);
    assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
}

struct Solution;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for (i, v) in arr.iter().enumerate() {
            map.entry(v).or_insert(vec![]).push(i);
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));
        let mut visited = vec![false; arr.len()];
        visited[0] = true;
        while let Some((i, step)) = queue.pop_front() {
            if i == arr.len() - 1 {
                return step;
            }
            if i > 0 && !visited[i - 1] {
                visited[i - 1] = true;
                queue.push_back((i - 1, step + 1));
            }
            if i < arr.len() - 1 && !visited[i + 1] {
                visited[i + 1] = true;
                queue.push_back((i + 1, step + 1));
            }
            if let Some(v) = map.get(&arr[i]) {
                for &j in v {
                    if !visited[j] {
                        visited[j] = true;
                        queue.push_back((j, step + 1));
                    }
                }
                map.remove(&arr[i]);
            }
        }
        return 0;
    }
}
