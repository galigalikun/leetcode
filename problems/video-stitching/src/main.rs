fn main() {
    assert_eq!(
        Solution::video_stitching(
            vec![
                vec![0, 2],
                vec![4, 6],
                vec![8, 10],
                vec![1, 9],
                vec![1, 5],
                vec![5, 9]
            ],
            10
        ),
        3
    );
    assert_eq!(
        Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5),
        -1
    );
    assert_eq!(
        Solution::video_stitching(
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9]
            ],
            9
        ),
        3
    );
}

struct Solution;
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut clips = clips;
        clips.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut dp = vec![0; time as usize + 1];
        for clip in clips {
            if clip[0] as usize <= time as usize && clip[1] as usize > dp[clip[0] as usize] {
                dp[clip[0] as usize] = clip[1] as usize;
            }
        }
        let mut last = 0;
        let mut ret = 0;
        let mut pre = 0;
        for i in 0..time as usize {
            last = last.max(dp[i]);
            if i == last {
                return -1;
            }
            if i == pre {
                ret += 1;
                pre = last;
            }
        }
        return ret;
    }
}
