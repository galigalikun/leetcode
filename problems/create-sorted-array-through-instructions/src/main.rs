fn main() {
    assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1);
    assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
    assert_eq!(
        Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]),
        4
    );
}

struct Solution;
impl Solution {
    fn get(bit: &Vec<i32>, mut x: i32) -> i32 {
        let mut sum = 0;
        while x > 0 {
            sum += bit[x as usize];
            x -= x & -x;
        }
        sum
    }
    fn update(bit: &mut Vec<i32>, mut x: i32, val: i32) {
        while x < bit.len() as i32 {
            bit[x as usize] += val;
            x += x & -x;
        }
    }
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut bit = vec![0; 100001];
        let mut cost = 0;
        let mut n = 0;
        let m = 1000000007;
        for &x in instructions.iter() {
            let greater = n - Self::get(&bit, x);
            let less = Self::get(&bit, x - 1);
            Self::update(&mut bit, x, 1);
            cost += std::cmp::min(less, greater);
            cost %= m;
            n += 1;
        }
        cost as i32
    }
}
