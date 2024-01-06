use crate::query::QueryValues;

/// A Nasa API Spec
pub trait Spec {
    /// The base url for the API
    const BASE_URL: &'static str;
    /// Query parameters for the API: Usually an enum
    type Params: serde::Serialize + Clone + Default;
    /// Response type for the API: Usually a serde_json::Value
    type ResponseType = serde_json::Value;
    /// Error type for the API
    type Error = Box<dyn std::error::Error>;

    /// reqwest::Response -> Self::ResponseType
    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType;
}

/// Core client functionality
pub trait ClientHandler<S: Spec>
where
    Self: Default,
{
    /// Build the query string
    fn build_query(params: S::Params) -> Result<String, crate::error::Error> {
        let mut url = S::BASE_URL.to_owned();
        url.push_str(
            &serde_qs::to_string(&params)
                .map_err(|e| crate::error::Error::SerializationError(e))?,
        );

        if let Some(key) = crate::prelude::try_api_key_from_env() {
            url.push_str(&format!("&api_key={}", key));
        } else {
            log::error!("No API key found in environment");
            return Err(crate::error::Error::ApiKeyError);
        }

        log::debug!("Built query bound for: {}", url);

        return Ok(url);
    }

    /// Query the API
    fn query(&self, params: &S::Params) -> Result<S::ResponseType, S::Error> {
        let url = Self::build_query(params.clone()).unwrap();
        let response = reqwest::blocking::Client::new()
            .get(&url)
            .send()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            .unwrap();
        let response = S::parse_response(response);
        Ok(response)
    }

    /// Query with generic params
    fn query_with(&self, params: impl QueryValues) -> Result<S::ResponseType, S::Error> {
        let values = params.values();
        let query = map_to_query(values);
        let url = S::BASE_URL.to_owned() + "?" + &query;
        let response = reqwest::blocking::Client::new()
            .get(&url)
            .send()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            .unwrap();
        let response = S::parse_response(response);
        Ok(response)
    }
}

/// An agnostic client that can be used to wrap any API
pub struct Client<S: Spec> {
    _spec: std::marker::PhantomData<S>,
}

impl<S> Default for Client<S>
where
    S: Spec,
{
    fn default() -> Self {
        Self {
            _spec: std::marker::PhantomData,
        }
    }
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

/// Try and read an API key from the environment
pub(crate) fn try_api_key_from_env() -> Option<String> {
    dotenv::dotenv().ok();
    match std::env::var("NASA_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}
