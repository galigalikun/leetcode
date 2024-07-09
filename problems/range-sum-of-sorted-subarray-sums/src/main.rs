fn main() {
    assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    assert_eq!(Solution::range_sum(vec![1,2,3,4], 4, 3, 4), 6);
    assert_eq!(Solution::range_sum(vec![1,2,3,4], 4, 1, 10), 50);
}

struct Solution;
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        let mut v = vec![];
        for i in 0..n {
            for j in i..n {
                v.push(nums[j as usize]);
            }
        }
        v.sort();
        for i in left-1..right {
            sum += v[i as usize];
        }
        return sum;
    }
}
