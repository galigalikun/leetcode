fn main() {
    assert_eq!(Solution::judge_point24(vec![4,1,8,7]), true);
    assert_eq!(Solution::judge_point24(vec![1,2,1,2]), false);
}

struct Solution{}
impl Solution {
    fn judge_point24_helper(nums: &Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                let mut nums = nums.clone();
                nums.remove(i);
                nums.remove(j);
                if Self::judge_point24_helper(&nums) {
                    return true;
                }
            }
        }
        false
    }
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut cards = cards;
        cards.sort();
        return Self::judge_point24_helper(&cards);
    }
}
