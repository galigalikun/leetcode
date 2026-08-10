fn main() {
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
        true
    );
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
        false
    );
}

struct Solution {}
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        if n == 0 {
            return true;
        }

        let mut visited = vec![false; n];
        let mut stack = vec![0_usize];
        visited[0] = true;

        while let Some(room) = stack.pop() {
            for &key in &rooms[room] {
                if let Ok(next_room) = usize::try_from(key) {
                    if next_room < n && !visited[next_room] {
                        visited[next_room] = true;
                        stack.push(next_room);
                    }
                }
            }
        }

        visited.into_iter().all(|is_visited| is_visited)
    }
}
