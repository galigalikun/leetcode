fn main() {
    assert_eq!(Solution::count_sub_trees(7, vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]], "abaedcd".to_string()), vec![2,1,1,1,1,1,1]);
    assert_eq!(Solution::count_sub_trees(4, vec![vec![0,1],vec![1,2],vec![0,3]], "bbbb".to_string()), vec![4,2,1,1]);
    assert_eq!(Solution::count_sub_trees(5, vec![vec![0,1],vec![0,2],vec![1,3],vec![0,4]], "aabab".to_string()), vec![3,2,1,1,1]);
}

struct Solution;
impl Solution {
    fn dfs(
        node: i32,
        graph: &Vec<Vec<i32>>,
        labels: &String,
        visited: &mut Vec<bool>,
        res: &mut Vec<i32>,
        count: &mut Vec<i32>,
    ) -> Vec<i32> {
        visited[node as usize] = true;
        let mut cur_count = vec![0; 26];
        cur_count[labels.chars().nth(node as usize).unwrap() as usize - 'a' as usize] = 1;
        for &child in graph[node as usize].iter() {
            if visited[child as usize] {
                continue;
            }
            let child_count = Self::dfs(child, graph, labels, visited, res, count);
            for i in 0..26 {
                cur_count[i] += child_count[i];
            }
        }
        count[cur_count[labels.chars().nth(node as usize).unwrap() as usize - 'a' as usize] as usize] += 1;
        res[node as usize] = count[cur_count[labels.chars().nth(node as usize).unwrap() as usize - 'a' as usize] as usize];
        return cur_count;
    }
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize];
        for e in edges {
            let a = e[0];
            let b = e[1];
            graph[a as usize].push(b);
            graph[b as usize].push(a);
        }
        let mut res = vec![0; n as usize];
        let mut visited = vec![false; n as usize];
        let mut count = vec![0; 26];
        return Self::dfs(0, &graph, &labels, &mut visited, &mut res, &mut count);
    }
}
