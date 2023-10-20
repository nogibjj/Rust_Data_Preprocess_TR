extern crate csv;
extern crate dataframe;

use csv::ReaderBuilder;
use dataframe::DataFrame;
use std::error::Error;
use std::fs::File;

fn read_csv_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    // Open the CSV file for reading
    let file = File::open(file_path)?;

    // Create a CSV reader with flexible options
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // Read the CSV data into a vector of records
    let records: Vec<csv::Result<csv::StringRecord>> = rdr.records().collect();

    // Check if there are any errors while reading the CSV
    for result in &records {
        result?;
    }

    // Convert the CSV records into a DataFrame
    let df = DataFrame::from_csv_records(records.iter().map(|r| r.as_ref().unwrap()))?;

    Ok(df)
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let file_path = "your_file.csv"; // Replace with the actual file path
//     let df = read_csv_to_dataframe(file_path)?;

//     // Now you can work with the DataFrame
//     println!("DataFrame:\n{}", df);

//     Ok(())
// }
