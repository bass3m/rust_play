extern crate rand;

use std::thread;
use std::sync::{Mutex,Arc};
use rand::Rng;

struct Philosopher {
    name : String,
    left : usize,
    right : usize,
}

impl Philosopher {
    fn new(name : &str, left : usize, right : usize) -> Philosopher {
        Philosopher {
            name : name.to_string(),
            left : left,
            right : right,
        }
    }
    fn eat(&self, table : &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        let num = rand::thread_rng().gen_range(1,101) * 100;
        println!("{} L {} R {} is eating for {} ms",self.name,self.left,self.right,num);
        thread::sleep_ms(num);
        println!("{} is done eating",self.name);
    }
}

struct Table {
    forks : Vec<Mutex<()>>,
}
fn main() {
    println!("Dining philosophers");
    let table = Arc::new(Table { forks : vec![Mutex::new(()),
                                              Mutex::new(()),
                                              Mutex::new(()),
                                              Mutex::new(()),
                                              Mutex::new(()),
                                              ]});
    let philos = vec![Philosopher::new("Spinoza",0,1),
                      Philosopher::new("Marx",1,2),
                      Philosopher::new("Camus",2,3),
                      Philosopher::new("Hegel",3,4),
                      Philosopher::new("Nietzsche",0,4),
                      ];
    let handles :  Vec<_> = philos.into_iter().map(|p| {
        let tbl = table.clone();

        thread::spawn(move || {
            p.eat(&tbl);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
