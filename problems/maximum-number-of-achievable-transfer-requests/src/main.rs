fn main() {
    assert_eq!(
        Solution::maximum_requests(
            5,
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![0, 1],
                vec![1, 2],
                vec![2, 0],
                vec![3, 4]
            ]
        ),
        5
    );
    assert_eq!(
        Solution::maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
        3
    );
    assert_eq!(
        Solution::maximum_requests(4, vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = requests.len();
        let mut ans = 0;
        for i in 0..1 << m {
            let mut cnt = vec![0; n];
            let mut tot = 0;
            for j in 0..m {
                if i >> j & 1 == 1 {
                    cnt[requests[j][0] as usize] -= 1;
                    cnt[requests[j][1] as usize] += 1;
                    tot += 1;
                }
            }
            if cnt.iter().all(|&x| x == 0) {
                ans = ans.max(tot);
            }
        }
        ans
    }
}
