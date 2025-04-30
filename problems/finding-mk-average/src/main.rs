struct MKAverage {
    m: i32,
    k: i32,
    elements: Vec<i32>,
    sum: i32,
    count: i32,
    min_heap: std::collections::BinaryHeap<i32>,
    max_heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {

    fn new(m: i32, k: i32) -> Self {
        MKAverage { m: m, k: k, elements: vec![], sum: 0, count: 0, min_heap: std::collections::BinaryHeap::new(), max_heap: std::collections::BinaryHeap::new() }
    }

    fn add_element(&mut self, num: i32) {
        if self.count < self.m {
            self.elements.push(num);
            self.sum += num;
            self.count += 1;
            if self.count == self.m {
                for _i in 0..self.k {
                    if let Some(min) = self.min_heap.pop() {
                        self.sum -= min;
                    }
                    if let Some(max) = self.max_heap.pop() {
                        self.sum -= max.0;
                    }
                }
            }
        } else {
            let min = *self.min_heap.peek().unwrap();
            let max = *self.max_heap.peek().unwrap();
            if num < min {
                self.sum -= min;
                self.min_heap.pop();
                self.min_heap.push(num);
                self.sum += num;
            } else if num > max.0 {
                self.sum -= max.0;
                self.max_heap.pop();
                self.max_heap.push(std::cmp::Reverse(num));
                self.sum += num;
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.count < self.m {
            return -1;
        }
        let mut sum = 0;
        for i in 0..self.m {
            sum += self.elements[i as usize];
        }
        let mut count = 0;
        for _i in 0..self.k {
            count += 1;
        }
        if count == 0 {
            return sum / (self.m - 2 * self.k);
        } else {
            return -1;
        }
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */
fn main() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3);
    obj.add_element(1);
    assert_eq!(-1, obj.calculate_mk_average());
    obj.add_element(10);
    assert_eq!(3, obj.calculate_mk_average());
    obj.add_element(5);
    obj.add_element(5);
    obj.add_element(5);
    assert_eq!(5, obj.calculate_mk_average());
}
