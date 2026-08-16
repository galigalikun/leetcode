use std::collections::BTreeSet;

struct ExamRoom {
    n: i32,
    occupied: BTreeSet<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            occupied: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.occupied.is_empty() {
            self.occupied.insert(0);
            return 0;
        }

        let mut best_seat = 0;
        let first = *self
            .occupied
            .first()
            .expect("occupied is guaranteed to be non-empty");
        let mut best_dist = first;

        let mut prev = first;
        for &current in self.occupied.iter().skip(1) {
            let dist = (current - prev) / 2;
            let candidate = prev + dist;
            if dist > best_dist {
                best_dist = dist;
                best_seat = candidate;
            }
            prev = current;
        }

        let tail_dist = (self.n - 1) - prev;
        if tail_dist > best_dist {
            best_seat = self.n - 1;
        }

        self.occupied.insert(best_seat);
        best_seat
    }

    fn leave(&mut self, p: i32) {
        self.occupied.remove(&p);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sequence() {
        let mut obj = ExamRoom::new(10);
        assert_eq!(0, obj.seat());
        assert_eq!(9, obj.seat());
        assert_eq!(4, obj.seat());
        assert_eq!(2, obj.seat());
        obj.leave(4);
        assert_eq!(5, obj.seat());
    }
}
