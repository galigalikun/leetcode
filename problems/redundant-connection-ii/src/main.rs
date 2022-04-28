fn main() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![1, 5]
        ]),
        vec![4, 1]
    );
}

// https://github.com/Seanforfun/Algorithm-and-Leetcode/blob/master/leetcode/685.%20Redundant%20Connection%20II.md
struct Solution {}
impl Solution {
    fn find(root: &mut Vec<i32>, node: i32) -> i32 {
        if root[node as usize] != node {
            root[node as usize] = Self::find(root, root[node as usize]);
        }
        return root[node as usize];
    }
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut root = vec![0; edges.len() + 1];
        let mut parent = vec![0; edges.len() + 1];
        for i in 1..root.len() {
            root[i] = i as i32;
        }
        let mut new_edges = edges.clone();
        let mut ans1: Option<Vec<i32>> = None;
        let mut ans2: Option<Vec<i32>> = None;
        for i in 0..edges.len() {
            let u = new_edges[i][0];
            let v = new_edges[i][1];
            if parent[v as usize] > 0 {
                ans1 = Some(vec![parent[v as usize], v]);
                ans2 = Some(vec![u, v]);
                new_edges[i][0] = -1;
                new_edges[i][1] = -1;
            }
            parent[v as usize] = u;
        }
        for edge in new_edges {
            let u = edge[0];
            let v = edge[1];
            if u < 0 || v < 0 {
                continue;
            }
            let p = Self::find(&mut root, u);
            let q = Self::find(&mut root, v);
            if p == q {
                if let Some(ans) = ans1 {
                    return ans;
                } else {
                    return edge;
                }
            }
            root[p as usize] = q;
        }
        return ans2.unwrap();
    }
}
