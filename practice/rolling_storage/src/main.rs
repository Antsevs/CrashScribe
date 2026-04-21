use sysinfo::{
    Components, Disks, Networks, System,
};
use std::thread;
use std::io;
use std::time::Duration;
use chrono::prelude::*;

struct TelemetrySample {
    timestamp: String, 
    cpu: f32, 
    used_memory: f32,
    total_memory: f32,
}

fn main() {
    let mut input = String::new();
    let time = Duration::new(1,0);  //1 second time declaration

    thread::spawn(move || {     //thread to get all telemetry into struct
        let mut sys = System::new_all();    //system created
        let mut db: Vec<TelemetrySample> = Vec::new();

        loop {
            let sample = capture_sample(&mut sys);

            println!("{}\n[CPU Usage] {:.2}%\n[Used Memory] {:.2}GiB\n[Total Memory] {:.2}GiB\n\n", 
                sample.timestamp, 
                sample.cpu, 
                sample.used_memory, 
                sample.total_memory);
            if db.len() < 10 {
                db.push(sample);
            } else {
                db.push(sample);
                db.remove(0);
            }
            thread::sleep(time);
        }
    });

    //grab exit on user submit
    while input.trim() != "exit" {
        input = get_exit();
    }
}

//capture all telemetry and return as struct
fn capture_sample(sys: &mut System) -> TelemetrySample {
    sys.refresh_all();  //refresh system 
    let timestamp = get_timestamp();
    let cpu = get_cpu_usage(sys);
    let used_memory = get_used_memory(sys);
    let total_memory = get_total_memory(sys);
    let sample = TelemetrySample {timestamp, 
        cpu, 
        used_memory, 
        total_memory};
    sample
}

//get average cpu usage
fn get_cpu_usage(sys: &mut System) -> f32 {
    let mut cpu_total:f32 = 0.0;
    for cpu in sys.cpus() {
        cpu_total += cpu.cpu_usage();
    }
    
    let cpu_avg = cpu_total / sys.cpus().len() as f32;
    cpu_avg
}

//get the unsigned 64 bit used memory value
fn get_used_memory(sys: &mut System) -> f32 {
    let used_memory_bytes = sys.used_memory();
    let used_memory_gibs = mem_in_gibs(used_memory_bytes);
    used_memory_gibs
}

//get the unsigned 64 bit total memory value
fn get_total_memory(sys: &mut System) -> f32 {
    let total_memory_bytes = sys.total_memory();
    let total_memory_gibs = mem_in_gibs(total_memory_bytes);
    total_memory_gibs
}

//get the formatted system timestamp
fn get_timestamp() -> String {
    let utc_readable = Utc::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    utc_readable
}

//convert bytes to GiB
fn mem_in_gibs(bytes: u64) -> f32 {
    let gibs = bytes as f32 / (1024.0 * 1024.0 * 1024.0);
    gibs
}

//exit function
fn get_exit() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}