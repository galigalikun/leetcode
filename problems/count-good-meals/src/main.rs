fn main() {
    assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
    assert_eq!(Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
}

struct Solution;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = vec![0; 22];
        let mut ans = 0;
        for &d in &deliciousness {
            for i in 0..22 {
                let target = 1 << i;
                if count.len() > (target - d) as usize {
                    ans += count[(target - d) as usize];
                }
            }
            if count.len() > d as usize {
                count[d as usize] += 1;
            }
        }
        ans % 1_000_000_007
    }
}
