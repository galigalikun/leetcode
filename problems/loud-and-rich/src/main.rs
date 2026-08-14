fn main() {
    assert_eq!(
        Solution::loud_and_rich(
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![3, 1],
                vec![3, 7],
                vec![4, 3],
                vec![5, 3],
                vec![6, 3]
            ],
            vec![3, 2, 5, 4, 6, 1, 7, 0]
        ),
        vec![5, 5, 2, 5, 4, 5, 6, 7]
    );
    assert_eq!(Solution::loud_and_rich(vec![], vec![0]), vec![0]);
}

struct Solution;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph = vec![Vec::new(); n];

        for relation in richer {
            let richer_person = relation[0] as usize;
            let poorer_person = relation[1] as usize;
            graph[poorer_person].push(richer_person);
        }

        let mut memo = vec![None; n];
        for person in 0..n {
            Self::dfs(person, &graph, &quiet, &mut memo);
        }

        memo
            .into_iter()
            .map(|best| best.unwrap_or(0) as i32)
            .collect()
    }

    fn dfs(person: usize, graph: &[Vec<usize>], quiet: &[i32], memo: &mut [Option<usize>]) -> usize {
        if let Some(answer) = memo[person] {
            return answer;
        }

        let mut best = person;
        for &richer_person in &graph[person] {
            let candidate = Self::dfs(richer_person, graph, quiet, memo);
            if quiet[candidate] < quiet[best] {
                best = candidate;
            }
        }

        memo[person] = Some(best);
        best
    }
}
