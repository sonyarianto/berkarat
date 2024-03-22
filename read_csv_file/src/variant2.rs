use csv;
use std::fs::File;
use std::io::BufReader;

pub fn process_csv() {
    // Open the CSV file
    let file = File::open("large.csv").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Include the header line, but can arrange with .has_headers(false or true)
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false) // Set to true if the CSV file has a header line
        .from_reader(reader);

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
