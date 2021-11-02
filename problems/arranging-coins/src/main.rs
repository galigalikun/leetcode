fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
    assert_eq!(Solution::arrange_coins(10), 4);
}

pub struct Solution {}
// https://scrapbox.io/rustacean/Arranging_Coins
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut a = n;
        let mut b = 0;
        while a - (b + 1) >= 0 {
            b += 1;
            a -= b;
        }

        return b;
    }
}
