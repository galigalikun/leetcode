struct SeatManager {
    // Define the structure to manage the seats
    // You can use a vector, a set, or any other data structure
    // to keep track of available and reserved seats.
    // For example:
    available_seats: Vec<i32>,
    reserved_seats: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            // Initialize the seat manager with n seats
            available_seats: (1..=n).collect(),
            reserved_seats: Vec::new(),
        }
    }

    fn reserve(&mut self) -> i32 {
        // Reserve the first available seat
        if !self.available_seats.is_empty() {
            // Remove the seat from available_seats and move it to reserved_seats
            let seat = self.available_seats.remove(0);
            self.reserved_seats.push(seat);
            seat
        } else {
            -1 // No seats available
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        // Unreserve the seat by moving it from reserved to available
        if let Some(pos) = self.reserved_seats.iter().position(|&x| x == seat_number) {
            // Move the seat from reserved to available
            self.available_seats.push(seat_number);
            self.reserved_seats.remove(pos);
        }
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
fn main() {
    let mut obj = SeatManager::new(5);
    assert_eq!(obj.reserve(), 1);
    assert_eq!(obj.reserve(), 2);
    obj.unreserve(2);
    assert_eq!(obj.reserve(), 2);
    assert_eq!(obj.reserve(), 3);
    assert_eq!(obj.reserve(), 4);
    assert_eq!(obj.reserve(), 5);
    obj.unreserve(5);
}
