fn main() {
    assert_eq!(Solution::find_the_winner(5, 2), 3);
    assert_eq!(Solution::find_the_winner(6, 5), 1);
}

struct Solution;
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut people: Vec<i32> = (1..=n).collect();
        let mut index = 0;
        while people.len() > 1 {
            index = (index + k as usize - 1) % people.len();
            people.remove(index);
        }
        people[0]
    }
}
