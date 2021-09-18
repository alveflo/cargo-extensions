mod crate_metadata_fetcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = crate_metadata_fetcher::get_crate_metadata("serde").await?;

    println!("Package name: {}, latest version: {}", result.name, result.vers);
    println!("Available features:");

    for (feature, _) in &result.features {
        println!("{}", feature);
    }
    Ok(())
}
