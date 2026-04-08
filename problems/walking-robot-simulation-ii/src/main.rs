struct Robot {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    dir: i32, // 0: North, 1: East, 2: South, 3: West
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Robot {
            width,
            height,
            x: 0,
            y: 0,
            dir: 1, // Start facing East
        }
    }

    fn step(&mut self, num: i32) {
        let mut steps = num;
        while steps > 0 {
            match self.dir {
                0 => { // North
                    let step = steps.min(self.height - self.y);
                    self.y += step;
                    steps -= step;
                }
                1 => { // East
                    let step = steps.min(self.width - self.x);
                    self.x += step;
                    steps -= step;
                }
                2 => { // South
                    let step = steps.min(self.y);
                    self.y -= step;
                    steps -= step;
                }
                3 => { // West
                    let step = steps.min(self.x);
                    self.x -= step;
                    steps -= step;
                }
                _ => unreachable!(),
            }
            if steps > 0 {
                self.dir = (self.dir + 1) % 4; // Turn right
            }
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }

    fn get_dir(&self) -> String {
        match self.dir {
            0 => "North".to_string(),
            1 => "East".to_string(),
            2 => "South".to_string(),
            3 => "West".to_string(),
            _ => unreachable!(),
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
fn main() {
    let mut obj = Robot::new(6, 3);
    obj.step(2);
    obj.step(2);
    assert_eq!(obj.get_pos(), vec![2, 0]);
    assert_eq!(obj.get_dir(), "East".to_string());
    obj.step(2);
    obj.step(1);
    obj.step(4);
    assert_eq!(obj.get_pos(), vec![1, 2]);
    assert_eq!(obj.get_dir(), "West".to_string());
}
