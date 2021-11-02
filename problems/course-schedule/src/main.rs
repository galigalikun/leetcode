fn main() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    assert_eq!(
        Solution::can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]),
        true
    );
}

pub struct Solution {}
// https://medium.com/@yzhua3/leetcode-course-schedule-642d91dbd425
use std::collections::HashMap;
impl Solution {
    fn helper(n: usize, graph: HashMap<usize, Vec<usize>>, visited: &mut Vec<usize>) -> bool {
        visited[n] = 1;
        if let Some(g) = graph.get(&n) {
            for &i in g {
                if visited[i] == 1 {
                    return false;
                }
                if visited[i] == 0 {
                    if !Solution::helper(i, graph.clone(), visited) {
                        return false;
                    }
                }
            }
        }
        visited[n] = 2;
        return true;
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        for p in prerequisites {
            if let Some(g) = graph.get_mut(&(p[1] as usize)) {
                (*g).push(p[0] as usize);
            } else {
                graph.insert(p[1] as usize, vec![p[0] as usize]);
            }
        }

        let mut visited = vec![0; num_courses as usize];
        for n in 0..num_courses as usize {
            if visited[n] == 0 {
                if !Solution::helper(n, graph.clone(), &mut visited) {
                    return false;
                }
            }
        }

        return true;
    }
}
