mod variant1;
// mod variant2;
// mod variant3;

#[tokio::main]
async fn main() {
    variant1::process_csv()
    // variant2::process_csv()
    // variant3::process_csv().await.expect("Failed to process CSV");
}
