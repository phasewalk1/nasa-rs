use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    /// default to 30 days prior to current UTC date
    start_date: Option<String>,
    /// default to current UTC date
    end_date: Option<String>,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
        }
    }
}

impl QueryValues for Params {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(start_date) = &self.start_date {
            map.insert("startDate".to_string(), start_date.to_owned());
        }
        if let Some(end_date) = &self.end_date {
            map.insert("endDate".to_string(), end_date.to_owned());
        }
        map
    }
}

pub struct Gst;

impl Spec for Gst {
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/GST";
    type Params = Params;

    fn build_query(params: &Self::Params) -> String {
        let mut url = Self::BASE_URL.to_string();
        let query = crate::prelude::map_to_query(params.values());
        url.push_str(&query);

        if let Some(key) = crate::prelude::try_api_key_from_env() {
            url.push_str(&format!("&api_key={}", key));
        } else {
            log::error!("No API key found in environment variable NASA_API_KEY");
        }

        url
    }

    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType {
        res.json().unwrap()
    }
}

impl ClientHandler<Gst> for Client<Gst> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gst() {
        let client = Client::<Gst>::default();
        let params = Params::default();
        let response = client.query(&params).unwrap();
    }
}
