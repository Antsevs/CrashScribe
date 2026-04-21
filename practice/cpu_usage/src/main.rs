use sysinfo::{
    Components, Disks, Networks, System,
};
use std::thread;
use std::io;
use std::time::Duration;
use chrono::prelude::*;

fn main() {
    let mut input = String::new();
    let time = Duration::new(1,0);  //1 second time declaration

    thread::spawn(move || {         //thread to print cpu usage
        let mut sys = System::new_all();    //system created
        loop {
            let cpu_avg = get_cpu_usage(&mut sys);
            println!("{}", cpu_avg);
            thread::sleep(time);
        }
    });

    while input.trim() != "exit" {
        input = get_exit();
    }
}

fn get_cpu_usage(sys: &mut System) -> f32 { //get cpu usage avg
    let mut cpu_total:f32 = 0.0;
    sys.refresh_cpu_usage();    //system refreshed
    for cpu in sys.cpus() {
        cpu_total += cpu.cpu_usage();
    }
    
    let cpu_avg = cpu_total / sys.cpus().len() as f32;
    cpu_avg
}

fn get_exit() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}