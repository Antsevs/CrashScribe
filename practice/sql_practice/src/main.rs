use rusqlite::{params, Connection, Result};
struct TelemetrySample {
    id: i32,
    timestamp: String, 
    cpu: f32, 
    used_memory: f32,
    total_memory: f32,
}

fn main() -> Result<()> {
    let conn = Connection::open("crashscribe.db")?;
    println!("Database opened successfully...");
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
    println!("Telemetry table ready...");

    let sample = TelemetrySample {
        id: 0,
        timestamp: "12:00".to_string(),
        cpu: 32.0,
        used_memory: 15.6,
        total_memory: 31.68,
    };
    conn.execute(
        "INSERT INTO telemetry (timestamp, cpu, used_memory, total_memory) VALUES (?1, ?2, ?3, ?4)",
        params![
            &sample.timestamp, 
            &sample.cpu, 
            &sample.used_memory, 
            &sample.total_memory
        ],
    )?;

    println!("Sample inserted!");
    Ok(())
}