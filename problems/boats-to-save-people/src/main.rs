fn main() {
    assert_eq!(Solution::num_rescue_boats(vec![1,2], 3), 1);
    assert_eq!(Solution::num_rescue_boats(vec![3,2,2,1], 3), 3);
    assert_eq!(Solution::num_rescue_boats(vec![3,5,3,4], 5), 4);
}

struct Solution;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut left = 0usize;
        let mut right = people.len();
        let mut boats = 0;

        while left < right {
            right -= 1;
            if people[left] + people[right] <= limit {
                left += 1;
            }
            boats += 1;
        }

        boats
    }
}
