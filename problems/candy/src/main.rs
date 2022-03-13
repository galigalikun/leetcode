fn main() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);

    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);

    assert_eq!(Solution::candy(vec![0]), 1);
}

struct Solution {}
// https://programmerstart.com/article/7768654943/
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        let mut count = vec![0; n];
        count[0] = 1;
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                count[i] = count[i - 1] + 1;
            } else {
                count[i] = 1;
            }
        }

        let mut result = count[n - 1];
        for i in (0..=n - 2).rev() {
            if ratings[i] > ratings[i + 1] && count[i] < count[i + 1] + 1 {
                count[i] = count[i + 1] + 1;
            }

            result += count[i];
        }
        return result;
    }
}
