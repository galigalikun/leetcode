fn main() {
    assert_eq!(Solution::max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]), 4);
    assert_eq!(
        Solution::max_happy_groups(4, vec![1, 3, 2, 5, 2, 2, 1, 6]),
        4
    );
}

struct Solution;
impl Solution {
    fn dfs(i: usize, happy: i32, mod_sum: i32, batch_size: i32, groups: &[i32]) -> i32 {
        if i == groups.len() {
            return happy + (mod_sum == 0) as i32;
        }
        let mut res = Self::dfs(i + 1, happy, mod_sum, batch_size, groups);
        let new_mod_sum = (mod_sum + groups[i] % batch_size) % batch_size;
        res = res.max(Self::dfs(
            i + 1,
            happy + (new_mod_sum == 0) as i32,
            new_mod_sum,
            batch_size,
            groups,
        ));
        res
    }
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        return Self::dfs(0, 0, 0, batch_size, &groups);
    }
}
