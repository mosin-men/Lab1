# Lab 1 - Stack Vector

Welcome! Please feel free to update this readme with any information you feel is relevant.

### Read on Pain of Mockery
Cargo throws a bunch of nasty crap all over the build directory. Before pushing a build, PLEASE, for the love of god, run `cargo clean` so you're only pushing source code and not a bunch of binaries and dynamic libraries that are compiled for various dependencies.

### ToDo
* As mentioned in notes below, Rust's `stdin` functionality is somewhat painful to use, but it needs to be added. The string READING is done, but the string PROCESSING is not.
* Validation of all tests.

### Notes
* Rust's `stdin` functionality blows.

### Questions for Class
Add an answer as a sublist or something as we receive them. Or maybe we can make some sort of Rust wiki so we all have future reference material. I for one plan to use this language at all my future internships to infuriate everyone I meet :-)

* If a function returns `Result<(), ()>`, how come `Ok(())` is a valid return expression, but `Err(())` is not? If you want to return any sort of error, the compiler seems to guide you towards using `return Err(());`.
