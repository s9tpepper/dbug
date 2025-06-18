# dbug

A tiny Rust debugging utility that is heavily inspired by the Node.js debug module (https://github.com/debug-js/debug).

## Installation
```
cargo add dbug
```

## Usage
`dbug` creates a logger instance using the name of your module/function which provides a `.log("message")` function that prints to stdout with a colorized contxt name. The names allow you to turn on/off different debug output across the application without commenting/uncommenting your log calls.o

## Examples

Given the code below:
```rust
use std::{thread::sleep, time::Duration};

use dbug::{Logger, dbug};

#[allow(unused)]
#[derive(Debug)]
struct Tester {
    thing: String,
}

fn main() {
    let debugger = Logger::new("label");
    debugger.log("hello world");
    debugger.log("hello world 2");

    // Simulate a slow function
    sleep(Duration::from_millis(158));

    let tester = Tester {
        thing: "is a hand".into(),
    };
    // This should log +158 ms since last log call
    debugger.log(&format!("hello world 3: {:?}", tester));

    // Use like format! or println!
    dbug!(debugger, "hello world 3.5: {:?}", tester);
    debugger.log("hello world 4");

    // Extend the logger to add more context to the prefix
    let extended = debugger.extend("extended");
    extended.log("extended hello world");

    // Add even more context to prefix
    let more_ext = extended.extend("deep");
    more_ext.log("more");

    // Use as closures
    let debugger = Logger::new("something");
    let log = debugger.to_closure();
    log("hello from something");

    let ext = debugger.extend("extended_again");
    let extended = ext.to_closure();
    extended("extended hello world");
}
```

### Enable all logs
`$ DEBUG=* cargo run`
![All logs](/images/all.png)

### Enable only "label" logs
`$ DEBUG=label cargo run`
![All label logs](/images/label_only.png)

### Enable all logs that start with "label"
`$ DEBUG=label* cargo run`
![All logs that start with label](/images/starts_with_label.png)

### Enable all logs except the ones from "label" module
`$ DEBUG=*,-label cargo run`
![All logs except label](/images/all_except_label.png)

### Enable all logs except the ones that start with "label" module
`$ DEBUG=*,-label* cargo run`
![All logs except starts with label](/images/all_except_starts_with_label.png)
