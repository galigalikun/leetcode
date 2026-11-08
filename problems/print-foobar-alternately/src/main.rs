use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct FooBar {
    n: usize,
    state: Arc<Mutex<bool>>, // false = foo's turn, true = bar's turn
    condvar: Arc<Condvar>,
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            n,
            state: Arc::new(Mutex::new(false)),
            condvar: Arc::new(Condvar::new()),
        }
    }

    fn foo<F: Fn()>(&self, print_foo: F) {
        for _ in 0..self.n {
            let mut state = self.state.lock().unwrap();
            // Wait until it's foo's turn (state == false)
            while *state {
                state = self.condvar.wait(state).unwrap();
            }
            print_foo();
            // Switch to bar's turn
            *state = true;
            self.condvar.notify_all();
        }
    }

    fn bar<F: Fn()>(&self, print_bar: F) {
        for _ in 0..self.n {
            let mut state = self.state.lock().unwrap();
            // Wait until it's bar's turn (state == true)
            while !*state {
                state = self.condvar.wait(state).unwrap();
            }
            print_bar();
            // Switch to foo's turn
            *state = false;
            self.condvar.notify_all();
        }
    }
}

fn main() {
    let n = 5;
    let foobar = Arc::new(FooBar::new(n));

    let foobar_clone = Arc::clone(&foobar);
    let foo_thread = thread::spawn(move || {
        foobar_clone.foo(|| {
            print!("foo");
        });
    });

    let bar_thread = thread::spawn(move || {
        foobar.bar(|| {
            print!("bar");
        });
    });

    foo_thread.join().unwrap();
    bar_thread.join().unwrap();
    println!(); // newline at the end
}
