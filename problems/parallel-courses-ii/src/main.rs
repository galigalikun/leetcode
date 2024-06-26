fn main() {
    assert_eq!(
        Solution::min_number_of_semesters(4, vec![vec![2, 1], vec![3, 1], vec![1, 4]], 2),
        3
    );
    assert_eq!(
        Solution::min_number_of_semesters(
            5,
            vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]],
            2
        ),
        4
    );
}

struct Solution;
impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut pre = vec![0; n];
        for r in relations {
            pre[r[1] as usize - 1] |= 1 << (r[0] - 1);
        }
        let mut dp = vec![n as i32; 1 << n];
        dp[0] = 0;
        for i in 0..1 << n {
            let mut can = 0;
            for j in 0..n {
                if i & pre[j] == pre[j] {
                    can |= 1 << j;
                }
            }
            can &= !i;
            let mut c = can;
            while c > 0 {
                if (c as i32).count_ones() as usize <= k {
                    dp[i | c] = dp[i | c].min(dp[i] + 1);
                }
                c = (c - 1) & can;
            }
        }
        return dp[(1 << n) - 1];
    }
}
