fn main() {
    assert_eq!(Solution::is_possible(vec![9,3,5]), true);
    assert_eq!(Solution::is_possible(vec![1,1,1,2]), false);
    assert_eq!(Solution::is_possible(vec![8,5]), true);
}

struct Solution;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut target = target;
        target.sort();
        let mut sum = target.iter().sum::<i32>();
        let mut max = target.len() as i32 - 1;
        while target[max as usize] > 1 {
            let x = target[max as usize];
            let y = sum - x;
            let z = x % y;
            if z == 0 {
                return false;
            }
            target[max as usize] = z;
            sum = y + z;
            max = target.len() as i32 - 1;
            target.sort();
        }
        return target[max as usize] == 1;
    }
}
