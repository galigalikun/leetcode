fn main() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
    assert_eq!(
        Solution::find_longest_chain(vec![
            vec![-6, 9],
            vec![1, 6],
            vec![8, 10],
            vec![-1, 4],
            vec![-6, -2],
            vec![-9, 8],
            vec![-5, 3],
            vec![0, 3]
        ]),
        3
    )
}

// https://www.geeksforgeeks.org/maximum-length-chain-of-pairs-set-2/
struct Solution {}
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pp = pairs;
        pp.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut ans = 1;
        let mut prev = pp[0][1];
        for i in 1..pp.len() {
            if pp[i][0] > prev {
                ans += 1;
                prev = pp[i][1];
            }
        }
        return ans;
    }
}
