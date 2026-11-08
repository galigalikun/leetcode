use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Foo {
    state: Arc<Mutex<u8>>,
    cond: Arc<Condvar>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            state: Arc::new(Mutex::new(0)),
            cond: Arc::new(Condvar::new()),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        let mut state = self.state.lock().unwrap();
        // State 0: ready to execute first
        print_first();
        // Move to state 1
        *state = 1;
        self.cond.notify_all();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let mut state = self.state.lock().unwrap();
        // Wait until first is done (state >= 1)
        while *state < 1 {
            state = self.cond.wait(state).unwrap();
        }
        print_second();
        // Move to state 2
        *state = 2;
        self.cond.notify_all();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let mut state = self.state.lock().unwrap();
        // Wait until second is done (state >= 2)
        while *state < 2 {
            state = self.cond.wait(state).unwrap();
        }
        print_third();
    }
}

fn main() {
    let foo = Arc::new(Foo::new());

    let foo1 = Arc::clone(&foo);
    let t1 = thread::spawn(move || {
        foo1.first(|| println!("first"));
    });

    let foo2 = Arc::clone(&foo);
    let t2 = thread::spawn(move || {
        foo2.second(|| println!("second"));
    });

    let foo3 = Arc::clone(&foo);
    let t3 = thread::spawn(move || {
        foo3.third(|| println!("third"));
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
