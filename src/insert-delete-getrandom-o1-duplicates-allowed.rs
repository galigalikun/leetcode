struct RandomizedCollection {
    data: Vec<i32>,
    rand: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedCollection {
            data: vec![],
            rand: 0,
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let b = if None == self.data.iter().position(|&x| x == val) {
            true
        } else {
            false
        };
        self.data.push(val);
        return b;
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(p) = self.data.iter().position(|&x| x == val) {
            self.data.remove(p);
            return true;
        }

        return false;
    }

    /** Get a random element from the collection. */
    fn get_random(&mut self) -> i32 {
        if self.rand >= self.data.len() {
            self.rand = 0;
        }
        if let Some(&n) = self.data.get(self.rand) {
            self.rand += 1;
            return n;
        }
        return 0;
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn main() {
    let mut obj = RandomizedCollection::new();
    let ret_1: bool = obj.insert(1);
    assert_eq!(ret_1, true);
    let ret_2: bool = obj.insert(1);
    assert_eq!(ret_2, false);
    let ret_3: bool = obj.insert(2);
    assert_eq!(ret_3, true);
    assert_eq!(obj.get_random(), 1); // getRandom should return 1 with the probability 2/3, and returns 2 with the probability 1/3.
    let ret_4: bool = obj.remove(1);
    assert_eq!(ret_4, true);
    assert_eq!(obj.get_random(), 2); // getRandom should return 1 and 2 both equally likely.
}
