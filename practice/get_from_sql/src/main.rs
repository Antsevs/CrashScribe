use sysinfo::{
    Components, Disks, Networks, System,
};
use std::thread;
use std::io;
use std::time::Duration;
use chrono::prelude::*;
use rusqlite::{params, Connection, Result};

struct TelemetrySample {
    timestamp: String, 
    cpu: f32, 
    used_memory: f32,
    total_memory: f32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    let time = Duration::new(1,0);  //1 second time declaration
    
    //initializing sql database
    let conn = Connection::open("crashscribe.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS telemetry (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            cpu REAL,
            used_memory REAL,
            total_memory REAL
        )",
        (),
    )?;

/*    thread::spawn(move || {
        
    })*/

    thread::spawn(move || {     //thread to put all telemetry into struct
        let mut sys = System::new_all();    //system created
        loop {
            let sample = capture_sample(&mut sys);
            //first print telemetry then inject
            println!("{}\n[CPU Usage] {:.2}%\n[Used Memory] {:.2}GB\n[Total Memory] {:.2}GB\n\n", 
                sample.timestamp, 
                sample.cpu, 
                sample.used_memory, 
                sample.total_memory);
            if let Err(e) = conn.execute(
                "INSERT INTO telemetry (timestamp, cpu, used_memory, total_memory) VALUES (?1, ?2, ?3, ?4)",
                params![
                    &sample.timestamp,
                    &sample.cpu,
                    &sample.used_memory,
                    &sample.total_memory
                ],
            ) {
                println!("DB insert failed: {}", e); 
            }
            if let Err(e) = conn.execute(
                "DELETE FROM telemetry
                WHERE id NOT IN (
                    SELECT id
                    FROM telemetry
                    ORDER BY id DESC
                    LIMIT 10
                )",
                [],
            ) {
                println!("DB delete failed: {}", e);
            }
            thread::sleep(time);
        }
    });

    //grab exit on user submit
    while input.trim() != "exit" {
        input = get_exit();
        if input.trim() == "report" {
            println!("exporting report...");
            let report = Connection::open("crashscribe.db")?;
            let mut stmt = report.prepare(
                "SELECT timestamp, cpu, used_memory, total_memory 
                FROM telemetry
                ORDER BY id DESC
                LIMIT 10
                ")?;
            let rows = stmt.query_map([], |row| {
                Ok(TelemetrySample {
                    timestamp: row.get(0)?,
                    cpu: row.get(1)?,
                    used_memory: row.get(2)?,
                    total_memory: row.get(3)?,
                })
            })?;

            for row in rows {
                let sample = row?;
                println!(
                    "{}\n[CPU Usage] {:.2}%\n
                    [Used Memory] {:.2}GB\n
                    [Total Memory] {:.2}GB\n",
                    sample.timestamp,
                    sample.cpu,
                    sample.used_memory,
                    sample.total_memory
                );
            }
        }
    }

    Ok(())
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