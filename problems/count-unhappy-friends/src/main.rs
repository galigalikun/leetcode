fn main() {
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
            vec![vec![0, 1], vec![2, 3]]
        ),
        2
    );
    assert_eq!(
        Solution::unhappy_friends(2, vec![vec![1], vec![0]], vec![vec![1, 0]]),
        0
    );
    assert_eq!(
        Solution::unhappy_friends(
            4,
            vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]],
            vec![vec![1, 3], vec![0, 2]]
        ),
        4
    );
}

struct Solution;
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let mut unhappy = 0;
        let pairs = pairs
            .iter()
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(i32, i32)>>();
        let mut prefer = vec![vec![0; n as usize]; n as usize];
        for i in 0..n as usize {
            for j in 0..n as usize {
                prefer[i][preferences[i][j] as usize] = j as i32;
            }
        }
        for i in 0..n as usize {
            for j in 0..n as usize {
                if i == j {
                    continue;
                }
                let (x, y) = pairs
                    .iter()
                    .find(|(x, y)| *x == i as i32 || *y == i as i32)
                    .unwrap();
                let (u, v) = pairs
                    .iter()
                    .find(|(u, v)| *u == j as i32 || *v == j as i32)
                    .unwrap();
                if prefer[i][j] < prefer[i][*x as usize] && prefer[j][i] < prefer[j][*u as usize] {
                    unhappy += 1;
                    break;
                }
                if prefer[i][j] < prefer[i][*y as usize] && prefer[j][i] < prefer[j][*v as usize] {
                    unhappy += 1;
                    break;
                }
            }
        }
        unhappy
    }
}
