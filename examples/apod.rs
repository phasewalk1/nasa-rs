use nasa_rs::{params::ApodParams, prelude::*, Apod};

fn main() {
    let client = Apod::default();
    // Use today's date
    let params = ApodParams::default();
    let response = client.query(&params).unwrap();
    println!("{}", response);
}
