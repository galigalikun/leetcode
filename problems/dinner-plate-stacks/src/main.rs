use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    non_full: BinaryHeap<Reverse<usize>>,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: Vec::new(),
            non_full: BinaryHeap::new(),
        }
    }

    fn trim_right_empty(&mut self) {
        while self.stacks.last().is_some_and(|stack| stack.is_empty()) {
            self.stacks.pop();
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(&Reverse(index)) = self.non_full.peek() {
            if index >= self.stacks.len() || self.stacks[index].len() == self.capacity {
                self.non_full.pop();
            } else {
                break;
            }
        }

        if let Some(Reverse(index)) = self.non_full.pop() {
            self.stacks[index].push(val);
            if self.stacks[index].len() < self.capacity {
                self.non_full.push(Reverse(index));
            }
            return;
        }

        self.stacks.push(vec![val]);
        let index = self.stacks.len() - 1;
        if self.stacks[index].len() < self.capacity {
            self.non_full.push(Reverse(index));
        }
    }

    fn pop(&mut self) -> i32 {
        self.trim_right_empty();
        let Some(last_index) = self.stacks.len().checked_sub(1) else {
            return -1;
        };

        let value = self.stacks[last_index]
            .pop()
            .expect("last stack is non-empty after trimming");
        if self.stacks[last_index].len() + 1 == self.capacity {
            self.non_full.push(Reverse(last_index));
        }
        self.trim_right_empty();
        value
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index >= self.stacks.len() || self.stacks[index].is_empty() {
            return -1;
        }

        let value = self.stacks[index]
            .pop()
            .expect("checked non-empty stack before pop");
        if self.stacks[index].len() + 1 == self.capacity {
            self.non_full.push(Reverse(index));
        }
        if index + 1 == self.stacks.len() {
            self.trim_right_empty();
        }
        value
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::DinnerPlates;

    #[test]
    fn leetcode_example() {
        let mut obj = DinnerPlates::new(2);
        obj.push(1);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        obj.push(5);
        assert_eq!(2, obj.pop_at_stack(0));
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
}
