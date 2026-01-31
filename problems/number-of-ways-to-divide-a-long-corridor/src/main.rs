fn main() {
    assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
    assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
    assert_eq!(Solution::number_of_ways("S".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let seats: Vec<usize> = corridor
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == 'S' { Some(i) } else { None })
            .collect();
        if seats.len() % 2 != 0 {
            return 0;
        }
        let mut ways = 1;
        for i in (1..seats.len()).step_by(2) {
            if let Some(next) = seats.get(i + 1) {
                if seats[i] + 1 == *next {
                    continue;
                }
            }
            let gap = seats[i + 1] - seats[i];
            ways = (ways * (gap + 1)) % 1_000_000_007;
        }
        ways as i32
    }
}
