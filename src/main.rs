use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    right: usize,
    left: usize
}

struct Table {
    chopsticks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, right: usize, left: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            right,
            left
        }
    }

    fn eat(&self, table: &Table) {
        println!("{} is thinking...", self.name);
        let _pickup_right_fork = table.chopsticks[self.right].lock().unwrap(); 
        let _pickup_left_fork = table.chopsticks[self.left].lock().unwrap(); 
        println!("{} is eating!", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} has eaten!\n", self.name);
    }
}

// Each philosopher must think independlty from the next, and therfore not talk to eachother.
// Therefore we can have each philosopher be on its on thread and have the forks be a shared
// Mutex that is locked and unlocked as philosophers eat and think.

fn main() {
    let table = Arc::new(Table { chopsticks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Plato", 4, 0),
        Philosopher::new("Aristole", 0, 1),
        Philosopher::new("John Locke", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Ralph Waldo Emerson", 3, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let t = Arc::clone(&table);
        thread::spawn(move || {
            p.eat(&t);
        })
    }).collect();

    for h in handles {
        h.join().expect("Could not join the threads");
    }
}
