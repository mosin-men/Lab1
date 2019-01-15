/* Includes and crate linkages */
use std::io;
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
        s
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
            Err(())
        }
        else {
            self.size -= 1;
            Ok(&mut self.buffer[self.size])
        }
    }
}

fn main() -> Result<(), ()> {
    println!("StackVec");
    let mut store: [f64; 5] = [0.0; 5];
    let mut s = StackVec::new(&mut store);
    
    let mut in_str = String::new();
    loop {
        /* We have to clear here. Thanks to the borrow checker, we cannot
           clear the string when its split vector is in scope, and adding an
           extra scope layer just to house the split vector makes the code
           unsightly. Clearning here prevents odd scoping issues. */
        in_str.clear();
        if let Ok(bytes) = io::stdin().read_line(&mut in_str) {
            let split_str = in_str.split(" ");
        }
        else {
            println!("Error reading line!");
            break;
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
