/// Enum --> K/V pairs
pub(crate) trait QueryValues {
    fn values(&self) -> std::collections::HashMap<String, String>;
}

/// Build a query string from a hashmap
pub(crate) fn map_to_query(params: std::collections::HashMap<String, String>) -> String {
    let mut url = String::new();
    let size = params.len();
    for (i, (key, value)) in params.into_iter().enumerate() {
        url.push_str(&key);
        url.push_str("=");
        url.push_str(&value);
        if i < size - 1 {
            url.push_str("&");
        }
    }
    url
}

/// A Nasa API Spec
pub trait Spec {
    /// The base url for the API
    const BASE_URL: &'static str;
    /// Query parameters for the API: Usually an enum
    type Params;
    /// Response type for the API: Usually a serde_json::Value
    type ResponseType = serde_json::Value;
    /// Error type for the API
    type Error = Box<dyn std::error::Error>;

    /// Build the query string
    fn build_query(params: &Self::Params) -> String;
    /// reqwest::Response -> Self::ResponseType
    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType;
}

/// Core client functionality
pub trait ClientHandler<SPEC: Spec>
where
    Self: Default,
{
    /// Query the API
    fn query(&self, params: &SPEC::Params) -> Result<SPEC::ResponseType, SPEC::Error> {
        let url = SPEC::build_query(params);
        let response = reqwest::blocking::Client::new()
            .get(&url)
            .send()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            .unwrap();
        let response = SPEC::parse_response(response);
        Ok(response)
    }
}

/// An agnostic client that can be used to wrap any API
pub struct Client<SPEC: Spec> {
    _spec: std::marker::PhantomData<SPEC>,
}

impl<SPEC> Default for Client<SPEC>
where
    SPEC: Spec,
{
    fn default() -> Self {
        Self {
            _spec: std::marker::PhantomData,
        }
    }
}

use crate::clients::Apod;

/// Try and read an API key from the environment
pub(crate) fn try_api_key_from_env() -> Option<String> {
    dotenv::dotenv().ok();
    match std::env::var("NASA_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}
