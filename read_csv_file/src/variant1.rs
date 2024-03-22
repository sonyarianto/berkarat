use csv;
use std::fs::File;
use std::io::BufReader;

pub fn process_csv() {
    // Open the CSV file
    let file = File::open("large.csv").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create a CSV reader (skip the header line)
    let mut csv_reader = csv::Reader::from_reader(reader);

    // Iterate through each row (record)
    for result in csv_reader.records() {
        let record = result.expect("Failed to parse record");

        // Print the fields of the row
        for field in record.iter() {
            print!("{} ", field);
        }
        println!();
    }
}
