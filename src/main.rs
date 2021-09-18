mod crate_metadata_fetcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate_metadata_fetcher::get_crate_metadata("tokio").await
}
