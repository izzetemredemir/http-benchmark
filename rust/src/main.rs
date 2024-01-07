use reqwest;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() -> Result<(), reqwest::Error> {
    let total_requests = 100;
    let url = "https://example.com";

    let client = Arc::new(reqwest::blocking::Client::new());
    let start = Instant::now();

    let mut success_count = 0;
    for _ in 0..total_requests {
        let client = Arc::clone(&client);

        let response = client.get(url).send()?;
        if response.status().is_success() {
            success_count += 1;
        }
    }

    let duration = start.elapsed();
    let average_duration = duration / total_requests as u32;

    println!("Total requests: {}", total_requests);
    println!("Successful responses: {}", success_count);
    println!("Total duration: {:?}", duration);
    println!("Average duration: {:?}", average_duration);

    Ok(())
}

