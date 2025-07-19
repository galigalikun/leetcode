fn main() {
    assert_eq!(Solution::wonderful_substrings("aba".to_string()), 4);
    assert_eq!(Solution::wonderful_substrings("aabb".to_string()), 9);
    assert_eq!(Solution::wonderful_substrings("he".to_string()), 2);
    assert_eq!(Solution::wonderful_substrings("ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string()), 4375050000);
}

struct Solution;
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = 0;
        let mut mask = 0;
        let mut freq = vec![0; 1024];
        freq[0] = 1; // Initialize the frequency of the empty mask
        for c in word.chars() {
            mask ^= 1 << (c as u8 - b'a'); // Toggle the bit corresponding to the character
            count += freq[mask]; // Count substrings with the same mask

            // Check for masks that differ by one bit (i.e., one character can be changed)
            for i in 0..10 {
                count += freq[mask ^ (1 << i)];
            }

            freq[mask] += 1; // Increment the frequency of the current mask
        }
        count as i64 // Return the total count as i64
    }
}
