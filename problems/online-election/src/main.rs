struct TopVotedCandidate {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {

    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        TopVotedCandidate {  }
    }

    fn q(&self, t: i32) -> i32 {
        return 0;
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
