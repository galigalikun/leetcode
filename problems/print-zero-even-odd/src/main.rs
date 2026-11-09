use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let zero_even_odd = Arc::new(ZeroEvenOdd::new(2));
    
    let zero_clone = Arc::clone(&zero_even_odd);
    let even_clone = Arc::clone(&zero_even_odd);
    let odd_clone = Arc::clone(&zero_even_odd);
    
    let t1 = thread::spawn(move || {
        zero_clone.zero(|x| print!("{}", x));
    });
    
    let t2 = thread::spawn(move || {
        even_clone.even(|x| print!("{}", x));
    });
    
    let t3 = thread::spawn(move || {
        odd_clone.odd(|x| print!("{}", x));
    });
    
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    
    println!(); // newline for clarity
}

struct ZeroEvenOdd {
    n: i32,
    state: Mutex<State>,
    cond: Condvar,
}

struct State {
    num: i32,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            state: Mutex::new(State {
                num: 0,
            }),
            cond: Condvar::new(),
        }
    }

    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let mut state = self.state.lock().unwrap();
        for i in 0..self.n {
            while state.num != 2 * i {
                state = self.cond.wait(state).unwrap();
            }
            print_number(0);
            state.num += 1;
            self.cond.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let mut state = self.state.lock().unwrap();
        for i in 1..=self.n {
            if i % 2 == 0 {
                while state.num != 2 * i - 1 {
                    state = self.cond.wait(state).unwrap();
                }
                print_number(i);
                state.num += 1;
                self.cond.notify_all();
            }
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        let mut state = self.state.lock().unwrap();
        for i in 1..=self.n {
            if i % 2 == 1 {
                while state.num != 2 * i - 1 {
                    state = self.cond.wait(state).unwrap();
                }
                print_number(i);
                state.num += 1;
                self.cond.notify_all();
            }
        }
    }
}
