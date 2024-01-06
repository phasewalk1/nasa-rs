use crate::{prelude::*, query::QueryValues};

/// Astronomy Picture of the Day
pub struct Apod;

/// Query parameters for the APOD API
#[derive(serde::Serialize, Debug, Clone)]
pub struct ApodParams {
    /// The date of the APOD image to retrieve
    date: Option<String>,
    /// The start date of the APOD images to retrieve
    start_date: Option<String>,
    /// The end date of the APOD images to retrieve
    end_date: Option<String>,
    /// The number of APOD images to retrieve
    count: Option<u32>,
    /// The thumbsize of the APOD image to retrieve
    thumbs: Option<bool>,
}

impl Default for ApodParams {
    fn default() -> Self {
        let today = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let today = chrono::NaiveDateTime::from_timestamp(today as i64, 0);
        let today_string = today.format("%Y-%m-%d").to_string();
        let today_string = today_string.as_str();
        Self {
            date: Some(today_string.to_owned()),
            start_date: None,
            end_date: None,
            count: None,
            thumbs: None,
        }
    }
}

impl QueryValues for ApodParams {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        match self {
            ApodParams {
                date: Some(date), ..
            } => {
                map.insert("date".to_owned(), date.to_owned());
            }
            ApodParams {
                start_date: Some(start_date),
                ..
            } => {
                map.insert("start_date".to_owned(), start_date.to_owned());
            }
            ApodParams {
                end_date: Some(end_date),
                ..
            } => {
                map.insert("end_date".to_owned(), end_date.to_owned());
            }
            ApodParams {
                count: Some(count), ..
            } => {
                map.insert("count".to_owned(), count.to_string());
            }
            ApodParams {
                thumbs: Some(thumbs),
                ..
            } => {
                map.insert("thumbs".to_owned(), thumbs.to_string());
            }
            _ => {}
        }
        map
    }
}

impl Spec for Apod {
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/apod?";
    type Params = ApodParams;

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
        let params = ApodParams::default();
        let response: Value = client.query(&params).unwrap();
        let prtty = serde_json::to_string_pretty(&response).unwrap();
        println!("{}", prtty);
    }
}
