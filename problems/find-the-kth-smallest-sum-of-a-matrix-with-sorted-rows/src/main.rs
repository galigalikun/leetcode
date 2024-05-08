fn main() {
    assert_eq!(
        Solution::kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5),
        7
    );
    assert_eq!(
        Solution::kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 9),
        17
    );
    assert_eq!(
        Solution::kth_smallest(vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]], 7),
        9
    );
}

struct Solution;
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for i in 0..mat.len() {
            sum += mat[i][0];
        }
        heap.push((sum, vec![0; mat.len()]));
        let mut visited = vec![vec![false; mat[0].len()]; mat.len()];
        visited[0][0] = true;
        for _ in 0..k - 1 {
            if let Some((sum, idx)) = heap.pop() {
                for i in 0..mat.len() {
                    let mut idx = idx.clone();
                    idx[i] += 1;
                    if idx[i] < mat[i].len() && !visited[i][idx[i]] {
                        visited[i][idx[i]] = true;
                        let sum = sum - mat[i][idx[i] - 1] + mat[i][idx[i]];
                        heap.push((sum, idx));
                    }
                }
            }
        }
        if let Some(a) = heap.pop() {
            return a.0;
        }
        return 0;
    }
}
