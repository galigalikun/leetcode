fn main() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), vec![2, 3]);
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]]), vec![1, 4]);
}

struct Solution{}
impl Solution {
    fn find(v:usize, parent: &mut Vec<usize>) -> usize {
        if v != parent[v] {
            parent[v] = Self::find(parent[v], parent);
        }
        return parent[v];
    }
    fn dfs(u:usize, v:usize, rank: &mut Vec<i32>, parent: &mut Vec<usize>) -> bool {
        let up = Solution::find(u, parent);
        let vp = Solution::find(v, parent);
        if up == vp {
            return false;
        }
        if rank[up] < rank[vp] {
            parent[up] = vp;
        } else {
            parent[vp] = up;
            rank[up] += 1;
        }
        return true;
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        for edge in edges.clone() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            if Solution::dfs(u-1, v-1, &mut rank, &mut parent) {
                return edge;
            }
        }
        return vec![];
    }
}
