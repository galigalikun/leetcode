fn main() {
    assert_eq!(Solution::max_dist_to_closest(vec![1,0,0,0,1,0,1]), 2);
    assert_eq!(Solution::max_dist_to_closest(vec![1,0,0,0]), 3);
    assert_eq!(Solution::max_dist_to_closest(vec![0,1]), 1);
}

struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut previous_person: Option<usize> = None;
        let mut max_distance = 0usize;

        for (index, seat) in seats.iter().enumerate() {
            if *seat == 1 {
                match previous_person {
                    None => {
                        max_distance = index;
                    }
                    Some(prev) => {
                        let distance = (index - prev) / 2;
                        max_distance = max_distance.max(distance);
                    }
                }
                previous_person = Some(index);
            }
        }

        if let Some(last_person) = previous_person {
            max_distance = max_distance.max(seats.len() - 1 - last_person);
        }

        max_distance as i32
    }
}
