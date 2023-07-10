fn main() {
    assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
    assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
}

struct Solution;
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut candies = candies;
        let mut result = vec![0; num_people as usize];
        let mut i = 0;
        while candies > 0 {
            result[i % num_people as usize] += std::cmp::min(candies, i as i32 + 1);
            candies -= i as i32 + 1;
            i += 1;
        }
        return result;
    }
}
