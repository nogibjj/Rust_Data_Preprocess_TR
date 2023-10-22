extern crate csv;
extern crate rust_tr; // Import your own crate

use csv::ReaderBuilder;
use rust_tr::calculate_median;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write; // Added this import
use std::process::Command;
use std::time::Instant;
use sys_info::mem_info;

fn record_to_md(file_name: &str, time: u128) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    let mut file = std::io::BufWriter::new(file);

    writeln!(file, "Elapsed time: {} microseconds\n\n", time)?;
    Ok(())
}

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

    let start_time = Instant::now();
    // Calculate and print the medians
    let range_median = calculate_median(&range_values);

    println!("Electric Range Median: {}", range_median); // Fixed the typo

    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    match record_to_md("rust_time.md", elapsed_time.as_micros()) {
        Ok(_) => {}
        Err(e) => println!("Error: {:?}", e),
    }

    println!(
        "Memory Usage: {}%",
        (mem_info.total - mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}
