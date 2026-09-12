fn main() {
    assert_eq!(Solution::cat_mouse_game(vec![vec![2,5],vec![3],vec![0,4,5],vec![1,4,5],vec![2,3],vec![0,2,3]]), 0);
    assert_eq!(Solution::cat_mouse_game(vec![vec![1,3],vec![0],vec![3],vec![0,2]]), 1);
}

struct Solution;
impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let max_turns = 2 * n * n;
        let graph_usize: Vec<Vec<usize>> = graph
            .iter()
            .map(|neighbors| neighbors.iter().map(|&v| v as usize).collect())
            .collect();
        let mut memo = vec![vec![vec![-1_i32; max_turns + 1]; n]; n];

        fn dfs(
            mouse: usize,
            cat: usize,
            turn: usize,
            max_turns: usize,
            graph: &Vec<Vec<usize>>,
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            const DRAW: i32 = 0;
            const MOUSE: i32 = 1;
            const CAT: i32 = 2;

            if turn == max_turns {
                return DRAW;
            }
            if mouse == 0 {
                return MOUSE;
            }
            if mouse == cat {
                return CAT;
            }
            if memo[mouse][cat][turn] != -1 {
                return memo[mouse][cat][turn];
            }

            let result = if turn % 2 == 0 {
                let mut outcome = CAT;
                for &next_mouse in &graph[mouse] {
                    let next = dfs(next_mouse, cat, turn + 1, max_turns, graph, memo);
                    if next == MOUSE {
                        outcome = MOUSE;
                        break;
                    }
                    if next == DRAW {
                        outcome = DRAW;
                    }
                }
                outcome
            } else {
                let mut outcome = MOUSE;
                for &next_cat in &graph[cat] {
                    if next_cat == 0 {
                        continue;
                    }
                    let next = dfs(mouse, next_cat, turn + 1, max_turns, graph, memo);
                    if next == CAT {
                        outcome = CAT;
                        break;
                    }
                    if next == DRAW {
                        outcome = DRAW;
                    }
                }
                outcome
            };

            memo[mouse][cat][turn] = result;
            result
        }

        dfs(1, 2, 0, max_turns, &graph_usize, &mut memo)
    }
}
