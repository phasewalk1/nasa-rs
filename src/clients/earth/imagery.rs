use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ParamsEarthImagery {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Width and height of image in degrees
    pub dim: f64,
    /// Date of image: default to closest available to today
    pub date: Option<String>,
}
