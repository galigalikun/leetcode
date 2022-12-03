fn main() {
    assert_eq!(Solution::fair_candy_swap(vec![1,1], vec![2,2]), vec![1,2]);
    assert_eq!(Solution::fair_candy_swap(vec![1,2], vec![2,3]), vec![1,2]);
    assert_eq!(Solution::fair_candy_swap(vec![2], vec![1,3]), vec![2,3]);
}

struct Solution;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0, 0];
        let alice_sum = alice_sizes.iter().sum::<i32>();
        let bob_sum = bob_sizes.iter().sum::<i32>();
        println!("debug {} {}", alice_sum, bob_sum);
        return answer;
    }
}
