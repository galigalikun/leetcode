fn main() {
    assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
    assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 2);
    assert_eq!(Solution::find_min_moves(vec![0, 2, 0]), -1);
}

struct Solution {}
// https://www.cnblogs.com/grandyang/p/6648557.html
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let sum = machines.iter().fold(0, |a, b| a + b);

        if sum % machines.len() as i32 != 0 {
            return -1;
        }
        let avg = sum / machines.len() as i32;
        let (mut ans, mut cnt) = (0, 0);
        for m in machines {
            cnt += m - avg;
            ans = std::cmp::max(ans, std::cmp::max(cnt.abs(), m - avg));
        }

        return ans;
    }
}
