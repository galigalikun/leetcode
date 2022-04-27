fn main() {
    assert_eq!(Solution::find_redundant_directed_connection(vec![vec![1,2],vec![1,3],vec![2,3]]), vec![2,3]);
    assert_eq!(Solution::find_redundant_directed_connection(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,1],vec![1,5]]), vec![4,1]);
}

struct Solution{}
impl Solution {
    fn dfs(x: i32, y: i32, parent: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> bool {
        visited[x as usize] = true;

        return false;
    }
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        for edge in edges.clone() {
            if Solution::dfs(edge[0], edge[1], &edges, &mut vec![]) {
                return edge;
            }
        }
        return vec![];
    }
}
