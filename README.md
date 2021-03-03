# stdin-readlines
rust lib: read lines from stdin
Sometimes, I want to read stdin input data, it may be several lines, I'd like it could stop when get the `EOF` flag. Also, if the inputed lines can be edit, it will better. So this function can do these things:  
    1. read stdin input data until get the `EOF`(or can set other flag str)  
    2. could show the inputed data  
    3. delete inputed line with line num  
    4. support to set the insert line index(such as insert new lines under line N)  

## example
When use stdin-readlines

```rust
extern crate stdin_readlines;

use stdin_readlines::*;

let mut input = String::new();

// Default use `EOF` as stop flag
if !stdin_readlines(&mut input) {
    println!("there must be something wrong!");
}

println!("{}", input);

// use `EOI` as stop flag
let eof = 'EOI';
stdin_readlines_with_end(&mut input, &eof);

// replace std::io::stdin().read_line(&buf);
if stdin_read_line(&mut input) {
    println!("read line from stdin: {}", input);
}
```
