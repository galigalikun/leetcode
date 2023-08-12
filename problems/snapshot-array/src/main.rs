struct SnapshotArray {
    data: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            data: vec![vec![0; length as usize]; 1],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.data[0][index as usize] = val;
    }

    fn snap(&mut self) -> i32 {
        self.data.push(self.data[self.data.len() - 1].clone());
        return 0;
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        return self.data[snap_id as usize][index as usize];
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
fn main() {
    let mut obj = SnapshotArray::new(3);
    obj.set(0, 5); // Set array[0] = 5
    assert_eq!(obj.snap(), 0); // Take a snapshot, return snap_id = 0
    obj.set(0, 6);
    assert_eq!(obj.get(0, 0), 5); // Get the value of array[0] with snap_id = 0, return 5
}
