use crate::prelude::*;

/// Astronomy Picture of the Day
pub struct Apod;

/// Query parameters for the APOD API
#[derive(Debug, Clone, Copy)]
pub enum Params {
    /// The date of the APOD image to retrieve
    Date(&'static str),
    /// The start date of the APOD images to retrieve
    StartDate(&'static str),
    /// The end date of the APOD images to retrieve
    EndDate(&'static str),
    /// The number of APOD images to retrieve
    Count(u32),
    /// The thumbsize of the APOD image to retrieve
    Thumbs(bool),
}

impl QueryValues for Params {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        match self {
            Params::Date(date) => {
                map.insert(String::from("date"), String::from(date.to_owned()));
            }
            Params::StartDate(start_date) => {
                map.insert(String::from("start_date"), String::from(start_date.to_owned()));
            }
            Params::EndDate(end_date) => {
                map.insert(String::from("end_date"), String::from(end_date.to_owned()));
            }
            Params::Count(count) => {
                map.insert(String::from("count"), count.to_string());
            }
            Params::Thumbs(thumbs) => {
                map.insert(String::from("thumbs"), thumbs.to_string());
            }
        }
        map
    }
}

impl Spec for Apod {
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/apod?";
    type Params = Params;

    fn build_query(params: &Self::Params) -> String {
        let mut url = String::from(Self::BASE_URL);
        let query = crate::prelude::map_to_query(params.values());
        url.push_str(&query);

        if let Some(key) = crate::prelude::try_api_key_from_env() {
            log::debug!("Loaded api key from environment.");
            log::debug!("api_key={}", key);
            url.push_str(&format!("&api_key={}", key));
        } else {
            log::error!(
                "error: no api key found in environment variables. Ensure 'NASA_API_KEY' is set"
            );
        }

        log::debug!("Built query bound for: {}", url);
        return url;
    }

    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType {
        res.json().unwrap()
    }
}

// Implement Client Handling for Apod
impl ClientHandler<Apod> for Client<Apod> {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_apod() {
        pretty_env_logger::try_init().ok();

        let client = Client::<Apod>::default();
        let params = Params::Date("2020-01-01");
        let response: Value = client.query(&params).unwrap();
        let prtty = serde_json::to_string_pretty(&response).unwrap();
        println!("{}", prtty);
    }
}
