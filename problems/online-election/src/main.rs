use std::collections::HashMap;

struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut vote_count: HashMap<i32, i32> = HashMap::new();
        let mut leaders: Vec<i32> = Vec::with_capacity(persons.len());
        let mut current_leader = -1;
        let mut current_max_votes = 0;

        for person in persons {
            let count = vote_count.entry(person).or_insert(0);
            *count += 1;

            if *count >= current_max_votes {
                current_max_votes = *count;
                current_leader = person;
            }

            leaders.push(current_leader);
        }

        TopVotedCandidate { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        match self.times.binary_search(&t) {
            Ok(index) => self.leaders[index],
            Err(index) if index > 0 => self.leaders[index - 1],
            Err(_) => self.leaders[0],
        }
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
fn main() {
    let obj = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(0, obj.q(3));
    assert_eq!(1, obj.q(12));
    assert_eq!(1, obj.q(25));
    assert_eq!(0, obj.q(15));
    assert_eq!(0, obj.q(24));
    assert_eq!(1, obj.q(8));
}
