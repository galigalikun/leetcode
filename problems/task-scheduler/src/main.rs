fn main() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        8
    );
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        6
    );
    assert_eq!(
        Solution::least_interval(
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2
        ),
        16
    );
}

struct Solution {}
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }
        let mut stack = Vec::new();
        let mut ans = 0;
        let mut chip = Vec::new();
        for task in tasks {
            if chip.iter().find(|&x| x == &task) == None {
                ans += 1;
                chip.push(task);
            } else {
                stack.push(task);
            }
            if chip.len() >= n as usize {
                chip.clear();
            }
        }
        println!("debug {:?}", stack);
        return ans;
    }
}
