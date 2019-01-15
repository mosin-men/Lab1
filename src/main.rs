/* Includes and crate linkages */
use std::io;
use std::io::Write; /* so we can have the trait for flush() */
#[macro_use] extern crate scan_fmt;

/* The stackvec macro with arms for storage only and 
 *      storage with list of items to push */
macro_rules! stackvec {
    ($storage:expr) => (
        {
            let ret = StackVec::new($storage);
            ret
        }
    );

    ($storage:expr, $($x:expr),*) => (
        {
            let mut ret = StackVec::new($storage);
            $(
                /*Might need some error handling here*/
                let rv = ret.push($x);
                rv.expect("Not enough storage for all elements to push in macro");

            )*
            ret
        }
    );
}

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
    let mut in_str = String::new();
    let mut s = stackvec!(&mut store);

    loop {
        print!("Enter a command ('quit' to quit): ");
        io::stdout().flush().unwrap(); /* panic if failure */

        let mut in_str = String::new();

        if let Err(_) = io::stdin().read_line(&mut in_str) { break; }
        else if in_str.len() == 0 { break; }

        if in_str.chars().last().unwrap() == '\n' { in_str.pop(); }
        let mut split_str: Vec<&str> = in_str.split(" ").collect();
        match split_str[0] {
            "quit"  => break,
            "print" => cmd_print(&s),
            /* Get an index. Ensure the command is properly formatted, then
               perform the actual get operation. */
            "get"   => {
                if split_str.len() != 2 {
                    println!("Missing index or invalid command format.");
                }
                else {
                    match split_str[1].parse::<usize>() {
                        Ok(idx)     => cmd_get(&s, idx),
                        _           => println!("Invalid index type."),
                    }
                }
            },
            "set"   => (),
            /* Push a new value to the back of the vector. Make sure the 
               command is properly formatted, then make sure that the provided
               value can be converted to an f64, then perform the push. */
            "push"  => {
                if split_str.len() != 2 {
                    println!("Missing value or invalid command format.");
                }
                else {
                    match split_str[1].parse::<f64>() {
                        Ok(val)     => {
                            if let Err(()) = s.push(val) {
                                println!("Vector is full.");
                            }
                        },
                        _           => println!("Invalid value or command format.")
                    }
                }
            },
            /* Pop a value. If it's already in the vector we know it's an f64,
               so there's no need for any aggressive type-checking here. &*/
            "pop"   => {
                if split_str.len() != 1 {
                    println!("Invalid command format.");
                }
                else {
                    if let Ok(val) = s.pop() {
                        println!("Popped {}.", val);
                    }
                    else {
                        println!("Vector is empty.");
                    }
                }
            },
            _       => println!("Invalid command."),
        }
    }

    Ok(())
}

fn cmd_print(vec : &StackVec<f64>) {
    if vec.size() == 0 {
        println!("Vector is empty.");
        return;
    }
    let mut i = 0;
    for item in vec.iter() {
        println!("[{:03}] = {:.*}", i, 4, item);
        i += 1;
    }
}

fn cmd_get(vec : &StackVec<f64>, idx : usize) {
    if idx >= vec.size(){
        println!("Invalid index #{}", idx);
    }
    let mut i = 0;
    for item in vec.iter() {
        if idx == i{
            println!("Value at {} = {:.*}", i, 4, item);
            break;
        }
        i += 1;
    }
}
