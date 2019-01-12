/* Includes and crate linkages */
use std::io;
use std::io::Write; /* so we can have the trait for flush() */
#[macro_use] extern crate scan_fmt;

/* The StackVec structure with its two member variables. */
struct StackVec<'a, T: 'a> {
    buffer: &'a mut [T],
    size: usize,
}

/* Functions for the StackVec structure. */
impl<'a, T> StackVec<'a, T> {
    /* Create a new StackVec from a supplied array. This is a static method. */
    fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        let s = StackVec {buffer: storage, size: 0};
        return s
    }

    /* Get the USED size of the vector */
    fn size(&self) -> usize {
        self.size
    }

    /* Get the MAX size of the vector */
    fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    /* Push an object to the rear of the vector. Fail and return Err if
       capacity has been reached, otherwise add data and increment used 
       size. */
    fn push(&mut self, data: T) -> Result<(), ()> {
        let sz = self.size;
        let max_sz = self.buffer.len();

        if sz == max_sz {
            return Err(());
        }

        self.buffer[sz] = data;
        self.size += 1;
        Ok(())
    }

    /* Get the data stored at the tail of the vector. Fail and return Err if
       vector is empty. Otherwise, return a mutable reference to the data in
       question and decrement the used size. */
    fn pop(&mut self) -> Result<& mut T, ()> {
        if self.size == 0 {
            return Err(());
        }

        self.size -= 1;
        Ok(&mut self.buffer[self.size])
    }

    fn iter(&'a self) -> StackVecIterator<'a, T> {
        StackVecIterator{ vector: self, location: 0 }
    }
}

struct StackVecIterator<'a, T: 'a> {
    vector: &'a StackVec<'a, T>, //The vector to iterate across.
    location: usize, //The element the iterator is currently on.
}

impl <'a, T: 'a> Iterator for StackVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.location >= self.vector.size {
            None
        } else {
            self.location += 1;
            Some(& self.vector.buffer[self.location - 1])
        }
    }
}

fn main() -> Result<(), ()> {
    println!("StackVec");
    let mut store: [f64; 5] = [0.0; 5];
    let mut s = StackVec::new(&mut store);

    loop {
        print!("Enter a command ('quit' to quit): ");
        io::stdout().flush().unwrap(); /* panic if failure */

        let mut in_str = String::new();

        if let Err(_) = io::stdin().read_line(&mut in_str) { break; }
        else if in_str.len() == 0 { break; }

        if in_str.chars().last().unwrap() == '\n' { in_str.pop(); }
        let mut split_str = in_str.split(" ");
        match split_str.nth(0).unwrap().as_ref() {
            /* nth(0) will consume that item */
            "quit"  => break,
            "print" => cmd_print(&s),
            "get"   => {
                if let Some(s_idx) = split_str.nth(0) {
                    let _idx : Option<usize> = match s_idx.parse::<usize>() {
                        Ok(val) => Some(val),
                        _       => None
                    };
                    if let Some(idx) = _idx {
                        cmd_get(&s, idx);
                    } else {
                        println!("Invalid index");
                    }
                } else {
                    println!("Missing index");
                }
            },
            "set"   => (),
            "push"  => {
                println!("Pushing something.");
                if let Some(val) = split_str.nth(0) {
                    let float_val : f64 = val.parse().unwrap();
                    s.push(float_val);
                    println!("Pushed back {}", val);
                } else {
                    println!("Didn't get a value to push.");
                }
            },
            "pop"   => (),
            _       => println!("Invalid command") 
        }
    }

    /* Test code. Uncomment to run some simple, automated tests. Otherwise, 
       use the above code to run in interactive mode. */
    /*
    let bsz = s.buffer_size();
    println!("{}", bsz);
    let sz2 = s.size();
    println!("{}", sz2);

    for i in 0..10 {
        if let Err(()) = s.push(i as f64) {
            println!("Vector is full");
        }
    }

    for _i in 0..10 {
        if let Ok(v) = s.pop() {
            println!("Popped {}", v);
        }
        else {
            println!("Vector is empty");
        }
    }
    */

    Ok(())
}

fn cmd_print(vec : &StackVec<f64>) {
    let mut i = 0;
    for item in vec.iter() {
        println!("[{:03}] = {}", i, item);
        i += 1;
    }
}

fn cmd_get(vec : &StackVec<f64>, idx : usize) {
    // ...
}
