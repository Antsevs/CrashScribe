use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use std::io;

/*fn main() {
    println!("Type 'exit' to stop");
    let mut input = String::new();

    thread::spawn(move || {
        loop {
            let time = Duration::new(1, 0);
            let utc_readable = Utc::now().format("%Y-%m-%d %H:%M:%S");
            println!("{}", utc_readable);
            thread::sleep(time);
        };
    });

    while input.trim() != "exit" {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

}*/

fn main() {
    println!("Type 'exit' to stop");
    let mut input = String::new();
    let time = Duration::new(1,0);

    thread::spawn(move || {
        print_time(time);
    });

    while input.trim() != "exit" {
        input = get_exit();
    }

}

fn print_time(time: Duration) {
    loop {
        let utc_readable = Utc::now().format("%Y-%m-%d %H:%M:%S");
        println!("[Clock Thread] {}", utc_readable);
        thread::sleep(time);
    }
}

fn get_exit() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}


