fn main() {
    assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
    assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
    assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
}

struct Solution;
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort();
        let mut i = 0;
        let mut j = piles.len() - 2;
        let mut sum = 0;
        while i < j {
            sum += piles[j];
            i += 1;
            if j > 1 {
                j -= 2;
            } else {
                break;
            }
        }
        sum
    }
}
