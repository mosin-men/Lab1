use std::io;
#[macro_use] extern crate scan_fmt;

struct StackVec<'a, T: 'a> {
    buffer: &'a mut [T],
    size: usize,
}
impl<'a, T> StackVec<'a, T> {
    fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        let s = StackVec {buffer: storage, size: 0};
        return s
    }

    fn size(&self) -> usize {
        self.size
    }

    fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    fn push(&mut self, data: T) -> Result<(), ()> {
        if self.size == self.buffer.len() {
            return Err(());
        }

        self.buffer[self.size] = data;
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<& mut T, ()> {
        if self.size == 0 {
            return Err(());
        }

        self.size -= 1;
        Ok(&mut self.buffer[self.size])
    }
}

fn main() -> Result<(), ()> {
    println!("Hello, world!");
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
    /*let bsz = s.buffer_size();
    println!("{}", bsz);
    let sz2 = s.size();
    println!("{}", sz2);

    for i in 0..10 {
        if let Err(()) = s.push(i) {
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
    }*/

    Ok(())
}
