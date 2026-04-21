use sysinfo::{
    Components, Disks, Networks, System,
};
use std::thread;
use std::io;
use chrono::prelude::*;

fn main() {

}

fn get_cpu() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

}
