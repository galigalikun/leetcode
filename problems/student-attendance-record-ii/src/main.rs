fn main() {
    assert_eq!(Solution::check_record(2), 8);
    assert_eq!(Solution::check_record(1), 3);
    assert_eq!(Solution::check_record(10101), 183236316);
}

// https://dreamume.medium.com/leetcode-552-student-attendance-record-ii-c0317c91f4b0
struct Solution {}
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut last_a = vec![0; 4];
        let mut last_l = vec![0; 4];
        let mut last_p = vec![0; 4];
        last_a[0] = 1;
        last_l[0] = 1;
        last_p[0] = 1;
        last_l[1] = 3;
        last_a[1] = 2;
        last_a[2] = 4;
        let m = 1000000007;
        for i in 1..n as usize {
            last_l[(i - 1) % 4] %= m;
            last_a[(i - 1) % 4] %= m;
            last_p[(i - 1) % 4] %= m;
            if i > 1 {
                last_l[i % 4] = ((last_a[(i - 1) % 4] + last_p[(i - 1) % 4]) % m
                    + (last_a[(i - 2) % 4] + last_p[(i - 2) % 4]) % m)
                    % m;
            }
            last_p[i % 4] =
                ((last_l[(i - 1) % 4] + last_a[(i - 1) % 4]) % m + last_p[(i - 1) % 4] % m) % m;
            if i > 2 {
                last_a[i % 4] =
                    ((last_a[(i - 1) % 4] + last_a[(i - 2) % 4]) % m + last_a[(i - 3) % 4] % m) % m;
            }
        }
        return ((last_a[(n as usize - 1) % 4] % m + last_l[(n as usize - 1) % 4] % m) % m
            + last_p[(n as usize - 1) % 4] % m)
            % m;
    }
}
