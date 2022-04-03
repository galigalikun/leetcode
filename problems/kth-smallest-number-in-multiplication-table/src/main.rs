fn main() {
    assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
    assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    assert_eq!(
        Solution::find_kth_number(9895, 28405, 100787757),
        31666344
    );
}

struct Solution {}
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut ans = vec![];
        for i in 1..=m {
            let mut a = i;
            for _j in 1..=n {
                ans.push(a);
                a += i;
            }
        }
        ans.sort();
        return ans[k as usize - 1];
    }
}
