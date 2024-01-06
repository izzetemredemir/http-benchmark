use reqwest;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://example.com/";

    let start = Instant::now(); // Zaman ölçümünü başlat

    let res = reqwest::get(url).await?;

    let duration = start.elapsed(); // Geçen süreyi hesapla

    println!("Status Code: {}", res.status());

    println!("Request completed in: {:?}", duration); // Süreyi ekrana yazdır

    Ok(())
}
