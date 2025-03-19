fn main() {
    assert_eq!(
        Solution::count_pairs(
            4,
            vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]],
            vec![2, 3]
        ),
        vec![6, 5]
    );
    assert_eq!(
        Solution::count_pairs(
            5,
            vec![
                vec![1, 5],
                vec![1, 5],
                vec![3, 4],
                vec![2, 5],
                vec![1, 3],
                vec![5, 1],
                vec![2, 3],
                vec![2, 5]
            ],
            vec![1, 2, 3, 4, 5]
        ),
        vec![10, 10, 9, 8, 6]
    );
}

struct Solution;
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut adj = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize - 1;
            let v = edge[1] as usize - 1;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut cnt = vec![0; n as usize];
        for i in 0..n as usize {
            cnt[i] = adj[i].len() as i32;
        }
        let mut sorted = cnt.clone();
        sorted.sort();
        let mut ans = vec![];
        for q in queries {
            let mut res = 0;
            for i in 0..n as usize {
                let mut l = 0;
                let mut r = n as usize;
                while l < r {
                    let m = l + (r - l) / 2;
                    if cnt[i] + sorted[m] > q {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
                if cnt[i] + sorted[l] > q {
                    res += n as i32 - l as i32;
                } else {
                    res += n as i32 - l as i32 - 1;
                }
                if cnt[i] * 2 > q {
                    res -= 1;
                }
            }
            ans.push(res / 2);
        }
        ans
    }
}
