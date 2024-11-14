fn main() {
    assert_eq!(Solution::max_profit(vec![2,5], 4), 14);
    assert_eq!(Solution::max_profit(vec![3,5], 6), 19);
    assert_eq!(Solution::max_profit(vec![2,8,4,10,6], 20), 110);
}

struct Solution;
impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut inventory = inventory;
        inventory.sort_unstable();
        let mut orders = orders;
        let mut profit = 0;
        let mut i = inventory.len() - 1;
        while orders > 0 {
            let mut count = 1;
            while i > 0 && inventory[i] == inventory[i - 1] {
                i -= 1;
                count += 1;
            }
            let diff = inventory[i] - if i > 0 { inventory[i - 1] } else { 0 };
            let take = diff * count;
            if orders >= take {
                profit += (inventory[i] + inventory[i - 1] + 1) * diff / 2 * count;
                orders -= take;
            } else {
                let q = orders / count;
                let r = orders % count;
                profit += (inventory[i] + inventory[i] - q + 1) * q / 2 * count + (inventory[i] - q) * r;
                orders = 0;
            }
            i -= 1;
        }
        profit %= 1_000_000_007;
        profit as i32
    }
}
