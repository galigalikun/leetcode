fn main() {
    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
}

struct Solution {}
// https://www.programcreek.com/2014/05/leetcode-h-index-java/
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut work = citations;
        work.sort();
        let mut result = 0;
        for i in (0..work.len()).rev() {
            let cnt = (work.len() - i) as i32;
            if work[i] >= cnt {
                result = cnt;
            } else {
                break;
            }
        }
        return result;
    }
}
