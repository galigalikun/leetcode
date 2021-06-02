struct MedianFinder {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder { data: vec![] }
    }

    fn add_num(&mut self, num: i32) {
        self.data.push(num);
        self.data.sort_by(|a, b| a.cmp(b));
    }

    fn find_median(&mut self) -> f64 {
        return if self.data.len()%2 == 0 {
            (self.data[self.data.len()/2-1]+self.data[self.data.len()/2]) as f64/2.0
        } else {
            self.data[self.data.len()/2] as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
fn main() {
    // [[],[6],[],[10],[],[2],[],[6],[],[5],[],[0],[],[6],[],[3],[],[1],[],[0],[],[0],[]]
    let mut obj = MedianFinder::new();
    obj.add_num(6);
    assert_eq!(obj.find_median(), 6.0);
    obj.add_num(10);
    assert_eq!(obj.find_median(), 8.0);
    obj.add_num(2);
    assert_eq!(obj.find_median(), 6.0);
    obj.add_num(6);
    assert_eq!(obj.find_median(), 6.0);
    obj.add_num(5);
    assert_eq!(obj.find_median(), 6.0);
    obj.add_num(0);
    assert_eq!(obj.find_median(), 5.5);
}
