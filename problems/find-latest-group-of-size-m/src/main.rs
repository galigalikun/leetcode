fn main() {
    assert_eq!(Solution::find_latest_step(vec![3,5,1,2,4], 1), 4);
    assert_eq!(Solution::find_latest_step(vec![3,1,5,4,2], 2), -1);
}

struct Solution;
impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let mut res = -1;
        let mut len = vec![0; n + 2];
        let mut cnt = vec![0; n + 1];
        for i in 0..n {
            let x = arr[i] as usize;
            let l = len[x - 1];
            let r = len[x + 1];
            len[x - l] = l + r + 1;
            len[x + r] = l + r + 1;
            cnt[l] -= 1;
            cnt[r] -= 1;
            cnt[l + r + 1] += 1;
            if cnt[m as usize] > 0 {
                res = i as i32 + 1;
            }
        }
        res
    }
}
