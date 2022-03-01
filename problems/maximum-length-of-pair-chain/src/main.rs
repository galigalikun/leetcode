fn main() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
}

struct Solution {}
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pp = pairs;
        pp.sort();
        let mut ans = 0;
        let mut prev = &pp[0];
        for i in 1..pp.len() {
            if pp[i][0] > prev[1] {
                ans += 1;
            }
            prev = &pp[i];
        }
        return ans;
    }
}
