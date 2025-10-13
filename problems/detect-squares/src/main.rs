struct DetectSquares {
    points: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {

    fn new() -> Self {
        DetectSquares {
            points: Vec::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        self.points.push(point);
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut count = 0;
        for p in &self.points {
            if p[0] == point[0] && p[1] == point[1] {
                count += 1;
            }
        }
        count
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
fn main() {
    let mut obj = DetectSquares::new();
    obj.add(vec![3, 10]);
    obj.add(vec![11, 2]);
    obj.add(vec![3, 2]);
    assert_eq!(obj.count(vec![11, 10]), 1);
    assert_eq!(obj.count(vec![14, 8]), 0);
    obj.add(vec![11, 2]);
    assert_eq!(obj.count(vec![11, 10]), 2);
}
