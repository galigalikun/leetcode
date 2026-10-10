fn main() {
    assert_eq!(Solution::max_turbulence_size(vec![9,4,2,10,7,8,8,1,9]), 5);
    assert_eq!(Solution::max_turbulence_size(vec![4,8,12,16]), 2);
    assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
}

struct Solution;
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 {
            return arr.len() as i32;
        }

        let mut up = 1;
        let mut down = 1;
        let mut ans = 1;

        for i in 1..arr.len() {
            if arr[i] > arr[i - 1] {
                up = down + 1;
                down = 1;
            } else if arr[i] < arr[i - 1] {
                down = up + 1;
                up = 1;
            } else {
                up = 1;
                down = 1;
            }

            ans = ans.max(up).max(down);
        }

        ans
    }
}
