fn main() {
    assert_eq!(
        Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::add_to_array_form(vec![2, 7, 4], 181),
        vec![4, 5, 5]
    );
    assert_eq!(
        Solution::add_to_array_form(vec![2, 1, 5], 806),
        vec![1, 0, 2, 1]
    );
    assert_eq!(
        Solution::add_to_array_form(vec![0], 10000),
        vec![1, 0, 0, 0, 0]
    );
    assert_eq!(
        Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        Solution::add_to_array_form(
            vec![1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3],
            516
        ),
        vec![1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 5, 7, 9]
    );
}

struct Solution;
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num = num;
        let mut k = k;
        let mut i = num.len() as i32 - 1;
        while k > 0 {
            if i < 0 {
                num.insert(0, 0);
                i = 0;
            }
            let mut sum = num[i as usize] + k % 10;
            k /= 10;
            if sum >= 10 {
                sum -= 10;
                k += 1;
            }
            num[i as usize] = sum;
            i -= 1;
        }
        num
    }
}
