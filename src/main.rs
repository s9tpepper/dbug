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
