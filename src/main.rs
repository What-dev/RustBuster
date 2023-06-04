#![allow(non_snake_case)]
use std::time::Instant;
use std::env;


fn main() -> Result<(), isahc::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a URL as an argument");
        return Ok(());
    }

    let url = args[1].trim().to_string();
    let mut url = url;

    if !url.starts_with("https://") {
        url = format!("https://{}", url);
    }

    println!("Checking {}...", url);
//request start
    let start = Instant::now();
    let response = isahc::get(url)?;
    let elapsed = start.elapsed();

//status checker
    if response.status().is_success() {
        println!("Website is up. Response time: {:?}", elapsed);
    } else {
        println!("Website is down. Response time: {:?}", elapsed);
    }

    Ok(())
}