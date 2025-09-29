fn main() {
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0]), 2);
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 1, 2, 0]), 6);
}

struct Solution;
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let mut visited = vec![false; next_visit.len()];
        let mut current_room = 0;
        let mut days = 0;
        let modulo = 1_000_000_007;
        while visited.iter().any(|&v| !v) {
            visited[current_room] = true;
            if days % 2 == 0 {
                current_room = next_visit[current_room] as usize;
            } else {
                current_room = (current_room + 1) % next_visit.len();
            }
            days = (days + 1) % modulo;
        }
        return days;
    }
}
