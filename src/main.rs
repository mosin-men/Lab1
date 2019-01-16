/* Includes and crate linkages */
use std::io;
use std::io::Write;                 /* so we can have the trait for flush() */
use std::ops::{Index, IndexMut};    /* For indexing */

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
        let max_sz = self.buffer_size();

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
    fn pop(&mut self) -> Result<&mut T, ()> {
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

/* Indexers for the StackVec. Should be pretty self-explanatory.
   Except for the second lifetime, that is. */
impl<'a, T> Index<usize> for StackVec<'a, T> {
    type Output = T;

    fn index<'b>(&'b self, index: usize) -> &'b T {
        &self.buffer[index]
    }
}

impl<'a, T> IndexMut<usize> for StackVec<'a, T> {
    fn index_mut<'b>(&'b mut self, index: usize) -> &'b mut T {
        &mut self.buffer[index]
    }
}

fn main() -> Result<(), ()> {
    println!("StackVec");
    let mut store: [f64; 5] = [0.0; 5];     // Storage array for StackVec
    let mut s = stackvec!(&mut store);      // I wonder what this is?

    loop {
        print!("Enter a command ('quit' to quit): ");
        io::stdout().flush().unwrap(); /* panic if failure */

        let mut in_str = String::new();

        if let Err(_) = io::stdin().read_line(&mut in_str) { break; }
        else if in_str.len() == 0 { break; }

        if in_str.chars().last().unwrap() == '\n' { in_str.pop(); }
        let split_str: Vec<&str> = in_str.split(" ").collect();
        match split_str[0] {
            "quit"  => break,
            "print" => cmd_print(&s),
            /* Get an index. Ensure the command is properly formatted, then
               perform the actual get operation. */
            "get"   => {
                if let Err(e) = cmd_get(&s, &split_str) {
                    println!("{}", e);
                }
            },
            /* Set the value stored at a specified index, if it's a valid
               index. */
            "set"   => {
                match cmd_set(&mut s, &split_str) {
                    Err(e)  => println!("{}", e),
                    Ok(v)   => println!("{}", v),
                }
            },
            /* Push a new value to the back of the vector. Make sure the 
               command is properly formatted, then make sure that the provided
               value can be converted to an f64, then perform the push. */
            "push"  => {
                match cmd_push(&mut s, &split_str) {
                    Err(e)  => println!("{}", e),
                    Ok(v)   => println!("Pushed back {}", v),
                }
            },
            /* Pop a value. If it's already in the vector we know it's an f64,
               so there's no need for any aggressive type-checking here. &*/
            "pop"   => {
                match cmd_pop(&mut s, &split_str) {
                    Ok(v)   => println!("Popped {}", v),
                    Err(e)  => println!("{}", e),
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

/* Get an element from the StackVec using our fancy-schmancy indexer */
fn cmd_get(vec : &StackVec<f64>, svec: &Vec<&str>) -> Result<f64, String> {
    /* Make sure the command is properly formatted.
       We need exactly two args. */
    if svec.len() != 2 {
        return Err("Improperly formatted command".to_string());
    }

    /* Convert the index to a usize or return Err if impossible. */
    let idx: usize = match svec[1].parse::<usize>() {
        Ok(v)   => v,
        _       => return Err("Could not convert supplied index to an integer.".to_string()),
    };

    /* Bounds-check */
    if idx >= vec.size(){
        return Err(format!("Invalid index #{}", idx));
    }

    /* Success output and return */
    println!("Value at {} = {}", idx, vec[idx]);
    Ok(vec[idx])
}

fn cmd_set(s: &mut StackVec<f64>, vec: &Vec<&str>) -> Result<f64, String> {
    /* Error check and get the index from the string vector */
    if vec.len() != 3 {
        return Err(String::from("Improperly formatted command."));
    }

    /* Convert index to integer, or return an error */
    let idx: usize = match vec[1].parse::<usize>() {
        Ok(v)   => v,
        _       => return Err("Could not convert supplied index to integer.".to_string()),
    };

    /* Convert supplied value to f64, or return an error */
    let val: f64 = match vec[2].parse::<f64>() {
        Ok(v)   => v,
        _       => return Err("Could not convert supplied value to f64.".to_string()),
    };
    
    /* Check to make sure the index is valid */
    if idx >= s.size() {
        return Err(format!("Invalid index #{}", idx));
    }

    /* Now perform the actual set */
    s[idx] = val;
    Ok(val)
}

fn cmd_push(s: &mut StackVec<f64>, vec: &Vec<&str>) -> Result<f64, String> {
    /* Check for proper command format. */
    if vec.len() != 2 {
        return Err("Improperly formatted command.".to_string());
    }

    /* Convert value to float. */
    let val: f64 = match vec[1].parse::<f64>() {
        Ok(v)   => v,
        _       => return Err("Could not convert supplied value to f64.".to_string()),
    };

    /* Perform the push */
    if s.push(val) == Err(()) {
        return Err("Vector is full.".to_string());
    }

    Ok(val)
}

fn cmd_pop<'a>(s: &'a mut StackVec<f64>, vec: &Vec<&str>) -> Result<&'a mut f64, String> {
    if vec.len() != 1 {
        return Err("Improperly formatted command.".to_string());
    }

    match s.pop() {
        Ok(v)   => Ok(v),
        Err(()) => Err("Vector is empty.".to_string()),
    }
}
