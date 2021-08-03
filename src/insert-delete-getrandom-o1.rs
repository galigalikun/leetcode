use rand::{thread_rng, Rng};
use std::collections::HashSet;
struct RandomizedSet {
    data: HashSet<i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            data: HashSet::new(),
            rng: thread_rng(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        return self.data.insert(val);
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        return self.data.remove(&val);
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let i: usize = self.rng.gen_range(0..self.data.len());
        if let Some(&n) = self.data.iter().nth(i) {
            return n;
        };
        return 0;
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn main() {
    let mut obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(1);
    assert_eq!(ret_1, true);
    let ret_2: bool = obj.remove(2);
    assert_eq!(ret_2, false);
    let ret_3: bool = obj.insert(2);
    assert_eq!(ret_3, true);
    obj.get_random(); // 1 or 2
    let ret_4: bool = obj.remove(1);
    assert_eq!(ret_4, true);
    let ret_5: bool = obj.insert(2);
    assert_eq!(ret_5, false);
    assert_eq!(obj.get_random(), 2);
}
