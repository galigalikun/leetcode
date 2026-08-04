fn main() {
    assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
    assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
    assert_eq!(Solution::num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
}

struct Solution {}
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut count = [0_i32; 121];
        for age in ages {
            count[age as usize] += 1;
        }

        let mut prefix = [0_i32; 121];
        for age in 1..=120 {
            prefix[age] = prefix[age - 1] + count[age];
        }

        let mut total = 0_i32;
        for age_x in 15..=120 {
            let c_x = count[age_x];
            if c_x == 0 {
                continue;
            }

            let min_age_y = age_x / 2 + 7;
            let eligible_y = prefix[age_x] - prefix[min_age_y];
            total += c_x * (eligible_y - 1);
        }

        total
    }
}
