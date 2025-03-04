fn main() {
    assert_eq!(Solution::get_coprimes(vec![2, 3, 3, 2], vec![vec![0, 1], vec![1, 2], vec![1, 3]]), vec![-1, 0, 0, 1]);
    assert_eq!(Solution::get_coprimes(vec![5,6,10,2,3,6,15], vec![vec![0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]]), vec![-1,0,-1,0,0,0,-1]);
}

struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
    fn dfs(node: usize, parent: i32, graph: &Vec<Vec<i32>>, nums: &Vec<i32>, ans: &mut Vec<i32>, seen: &mut Vec<Vec<i32>>) {
        let mut max_depth = -1;
        for i in 1..51 {
            if Self::gcd(nums[node], i) == 1 && !seen[i as usize].is_empty() {
                let (depth, index) = seen[i as usize].last().unwrap();
                if *depth > max_depth {
                    max_depth = *depth;
                    ans[node] = *index;
                }
            }
        }
        let temp = seen[nums[node] as usize].clone();
        seen[nums[node] as usize].push((temp.len() as i32, node as i32));
        for &n in &graph[node] {
            if n != parent {
                Self::dfs(n as usize, node as i32, graph, nums, ans, seen);
            }
        }
        seen[nums[node] as usize] = temp;
    }
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut graph = vec![vec![]; n];
        for n in edges {
            graph[n[0] as usize].push(n[1]);
            graph[n[1] as usize].push(n[0]);
        }
        let mut ans = vec![-1; n];
        let mut seen = vec![vec![]; 51];
        Self::dfs(0, -1, &graph, &nums, &mut ans, &mut seen);
        ans
    }
}
