struct DinnerPlates {
    capacity: i32,
    stacks: Vec<Vec<i32>>,
    left: i32,
    right: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        DinnerPlates {
            capacity: capacity,
            stacks: vec![],
            left: 0,
            right: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let mut index = self.left;
        while index < self.stacks.len() as i32
            && self.stacks[index as usize].len() == self.capacity as usize
        {
            index += 1;
        }
        if index == self.stacks.len() as i32 {
            self.stacks.push(vec![]);
        }
        self.stacks[index as usize].push(val);
        self.left = index;
        self.right = self.right.max(index + 1);
    }

    fn pop(&mut self) -> i32 {
        let mut index = self.right;
        while index >= 0 && self.stacks[index as usize].len() == 0 {
            index -= 1;
        }
        if index < 0 {
            return -1;
        }
        let val = self.stacks[index as usize].pop().unwrap_or(-1);
        self.left = self.left.min(index);
        self.right = index;
        return val;
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        if index >= self.stacks.len() as i32 || self.stacks[index as usize].len() == 0 {
            return -1;
        }
        let val = self.stacks[index as usize].pop().unwrap_or(-1);
        self.left = self.left.min(index);
        self.right = self.right.max(index + 1);
        return val;
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */
fn main() {
    let mut obj = DinnerPlates::new(2);
    obj.push(1);
    obj.push(2);
    obj.push(3);
    obj.push(4);
    obj.push(5);
    assert_eq!(4, obj.pop_at_stack(0));
    obj.push(20);
    obj.push(21);
    assert_eq!(20, obj.pop_at_stack(0));
    assert_eq!(21, obj.pop_at_stack(2));
    assert_eq!(5, obj.pop());
    assert_eq!(4, obj.pop());
    assert_eq!(3, obj.pop());
    assert_eq!(1, obj.pop());
    assert_eq!(-1, obj.pop());
}
