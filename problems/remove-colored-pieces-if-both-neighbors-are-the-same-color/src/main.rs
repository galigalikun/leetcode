fn main() {
    assert_eq!(Solution::winner_of_game("AAABABB".to_string()), true);
    assert_eq!(Solution::winner_of_game("AA".to_string()), false);
    assert_eq!(Solution::winner_of_game("ABBBBBBBAAA".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        return colors
            .as_bytes()
            .windows(3)
            .fold((0, 0), |(a, b), w| match w {
                b"AAA" => (a + 1, b),
                b"BBB" => (a, b + 1),
                _ => (a, b),
            })
            .0
            > colors
                .as_bytes()
                .windows(3)
                .fold((0, 0), |(a, b), w| match w {
                    b"AAA" => (a + 1, b),
                    b"BBB" => (a, b + 1),
                    _ => (a, b),
                })
                .1;
    }
}
