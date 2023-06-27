fn main() {
    assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
}

struct Solution;
impl Solution {
    fn dfs(count: &mut Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..26 {
            if count[i] == 0 {
                continue;
            }
            sum += 1;
            count[i] -= 1;
            sum += Self::dfs(count);
            count[i] += 1;
        }
        return sum;
    }
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut count = vec![0; 26];
        for c in tiles.chars() {
            count[(c as u8 - b'A') as usize] += 1;
        }
        return Self::dfs(&mut count);
    }
}
