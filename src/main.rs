extern crate csv;

use csv::ReaderBuilder;
use rust_tr::calculate_median;
use std::error::Error;
use std::fs::File;
use std::process::Command;
use std::time::Instant;
use sys_info::mem_info;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
    let start_time = Instant::now();
    let csv_file = "Electric_Vehicle_Population_Data.csv"; // Update with your CSV file path
    let file = File::open(csv_file)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let mut range_values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let range_v: f64 = record[10].parse()?;
        range_values.push(range_v);
    }

    // Calculate and print the medians
    let range_median = calculate_median(&range_values);

    println!("ELetirc Range Median: {}", range_median);

    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    println!(
        "Memory Usage: {}%",
        mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    println!("Elapsed time: {:?}", elapsed_time);
    // println!("Memory usage: {} bytes", std::mem::size_of::<f64>());

    Ok(())
}
