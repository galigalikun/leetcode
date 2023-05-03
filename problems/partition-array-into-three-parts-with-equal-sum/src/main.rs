fn main() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
        false
    );
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
        true
    );
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![1, -1, 1, -1]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = 0;
        for i in &arr {
            sum += i;
        }
        if sum % 3 != 0 {
            return false;
        }
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut sum3 = 0;
        let mut i = 0;
        let mut j = arr.len() - 1;
        while i < j {
            if sum1 != sum / 3 {
                sum1 += arr[i];
                i += 1;
            }
            if sum3 != sum / 3 {
                sum3 += arr[j];
                j -= 1;
            }
            if sum1 == sum / 3 && sum3 == sum / 3 {
                for k in i..j + 1 {
                    sum2 += arr[k];
                }
                if sum2 == sum / 3 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        return false;
    }
}
