use crate::{prelude::*, query::QueryValues};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum NeoParams {
    /// Retrieve a list of Asteroids based on their closest approach date to Earth.
    Feed {
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    },
    /// Lookup a specific Asteroid based on its [NASA JPL small body (SPK-ID) ID](http://ssd.jpl.nasa.gov/sbdb_query.cgi)
    Lookup { asteroid_id: i32 },
    /// Browse the overall Asteroid data-set
    Browse,
}

impl Default for NeoParams {
    fn default() -> Self {
        Self::Feed {
            start_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap()),
        }
    }
}

/// Asteroids Near-Earth Objects
pub struct Neo;

impl QueryValues for NeoParams {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        match self {
            Self::Feed {
                start_date,
                end_date,
            } => {
                if let Some(start_date) = start_date {
                    map.insert("start_date".to_string(), start_date.to_string());
                }
                if let Some(end_date) = end_date {
                    map.insert("end_date".to_string(), end_date.to_string());
                }
            }
            Self::Lookup { asteroid_id } => {
                map.insert("asteroid_id".to_string(), asteroid_id.to_string());
            }
            Self::Browse => {}
        }
        map
    }
}

impl Spec for Neo {
    const BASE_URL: &'static str = "https://api.nasa.gov/neo/rest/v1/";
    type Params = NeoParams;

    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType {
        res.json().unwrap()
    }
}

/// Implement the client handler for Neo
impl ClientHandler<Neo> for Client<Neo> {
    fn build_query(params: <Neo as Spec>::Params) -> Result<String, crate::error::Error> {
        let mut url = <Neo as Spec>::BASE_URL.to_string();
        match params {
            NeoParams::Feed { .. } => {
                url.push_str("feed?");
                let query = crate::prelude::map_to_query(params.values());
                url.push_str(&query);
            }
            NeoParams::Lookup { asteroid_id } => {
                url.push_str("neo/");
                url.push_str(&asteroid_id.to_string());
            }
            NeoParams::Browse => url.push_str("neo/browse?"),
        }

        if let Some(key) = crate::prelude::try_api_key_from_env() {
            log::debug!("Loaded API key from environment");
            url.push_str(&format!("&api_key={}", key));
        } else {
            log::error!("No API key found in environment variable NASA_API_KEY");
        }

        log::debug!("Built query bound for: {}", url);

        Ok(url)
    }
}

#[cfg(test)]
mod test_neo {
    use super::*;

    #[test]
    fn test_neo() {
        pretty_env_logger::try_init().ok();

        let client = Client::<Neo>::default();
        let params = NeoParams::default();
        let response = client.query(&params).unwrap();
        let prtty = serde_json::to_string_pretty(&response).unwrap();
        println!("{}", prtty);
    }
}
