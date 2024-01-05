use nasa_rs::clients::{Apod, ApodParams};
use nasa_rs::prelude::{Client, *};

fn main() {
    let client = Client::<Apod>::default();
    let params = ApodParams::Date("2020-01-01");
    let response = client.query(&params).unwrap();
    println!("{}", response);
}
