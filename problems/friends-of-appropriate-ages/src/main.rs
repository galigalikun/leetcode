fn main() {
    assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
    assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
}

struct Solution {}
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, &age1) in ages.iter().enumerate() {
            for (j, &age2) in ages.iter().enumerate() {
                if i != j {
                    if age1 as f64 > 0.5 * age2 as f64 + 7f64
                        && age1 < age2
                        && (age1 <= 100 || age2 > 100)
                    {
                        ans += 1;
                    }
                }
            }
        }
        return ans;
    }
}
