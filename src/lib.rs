//! nasa-rs provides client wrappers for
//! [NASA's open APIs](https://api.nasa.gov/)
//! > Example using the Astronomy Picture of the Day (APOD) API
//! ```rust
//! use nasa_rs::{
//!    prelude::*,
//!    Apod,
//!    params::ApodParams,
//! };
//!
//! fn main() {
//!    let client = Apod::default();
//!    // use today's date
//!    let params = ApodParams::default();
//!    let res = client.query(&params);
//!    assert!(res.is_ok());
//! }
//!
//! ```
#![feature(associated_type_defaults, associated_type_bounds)]
#![allow(unused_parens)]

/// Client implementations
pub mod clients;
/// Error types
pub mod error;
/// Common Parameters
pub mod params;
/// Common interfaces
pub mod prelude;
/// Interfaces for Query values
pub(crate) mod query;

/// Client handler for the Astronomy Picture of the Day (APOD) API
pub type Apod = prelude::Client<clients::Apod>;
/// Client handler for the Near Earth Object Web Service (NeoWs) API
pub type NeoWs = prelude::Client<clients::Neo>;
/// Client handler for the Geomagnetic Storm (GST) API
pub type Gst = prelude::Client<clients::Gst>;
/// Client handler for the CME Analysis (CME) API
pub type CmeAnalysis = prelude::Client<clients::CmeAnalysis>;
/// Client handler for the Solar Energetic Particle (SEP) API
pub type Flr = prelude::Client<clients::Flr>;
/// Client handler for the Solar Energetic Particle (SEP) API
pub type Sep = prelude::Client<clients::Sep>;

pub(crate) mod macros {
    #[macro_export]
    macro_rules! create_client_impl {
        ($client:ident, $base_url:expr, $params_type:ty) => {
            pub struct $client;

            impl $crate::prelude::Spec for $client {
                const BASE_URL: &'static str = $base_url;
                type Params = $params_type;

                fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType {
                    res.json().unwrap()
                }
            }

            impl ClientHandler<$client> for Client<$client> {}
        };
    }
}

#[cfg(test)]
mod test {
    use crate::clients::Apod;
    use crate::prelude::*;

    #[test]
    fn query_with() {
        let client = Client::<Apod>::default();
        let response = client
            .query_with(vec![("date", "2019-01-01"), ("hd", "true")])
            .map_err(|e| log::error!("{}", e))
            .unwrap();
        log::debug!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}
