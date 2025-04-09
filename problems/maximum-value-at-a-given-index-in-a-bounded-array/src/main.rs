fn main() {
    assert_eq!(Solution::max_value(4, 2, 6), 2);
    assert_eq!(Solution::max_value(6, 1, 10), 3);
    assert_eq!(Solution::max_value(3, 2, 18), 7);
}

struct Solution;
impl Solution {
    fn binary_search(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut left = 1;
        let mut right = max_sum;
        let mut ans = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let sum = Self::get_sum(mid, index, n);

            if sum <= max_sum {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
    fn get_sum(mid: i32, index: i32, n: i32) -> i32 {
        let left = index;
        let right = n - index - 1;
        let mut sum = 0;

        if left < mid {
            sum += (mid * (mid + 1)) / 2 - ((mid - left) * (mid - left + 1)) / 2;
        } else {
            sum += (mid * (mid + 1)) / 2;
        }

        if right < mid {
            sum += (mid * (mid + 1)) / 2 - ((mid - right) * (mid - right + 1)) / 2;
        } else {
            sum += (mid * (mid + 1)) / 2;
        }

        sum
    }
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        return Self::binary_search(n, index, max_sum);
    }
}
