use csv;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn process_csv() -> Result<(), std::io::Error> {
    // Open the CSV file
    let mut file = File::open("large.csv").await.expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.expect("Failed to read file");

    // Create a CSV reader (skip the header line)
    let mut csv_reader = csv::Reader::from_reader(buffer.as_slice());

    // Iterate through each row (record)
    for result in csv_reader.records() {
        let record = result.expect("Failed to parse record");

        // Print the fields of the row
        for field in record.iter() {
            print!("{} ", field);
        }
        println!();
    }

    Ok(())
}
