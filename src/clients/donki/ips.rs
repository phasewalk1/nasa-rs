#![allow(dead_code)]
pub use super::exports::Ips;

#[derive(Debug, Clone, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Location {
    Earth,
    MESSENGER,
    STEREO_A,
    STEREO_B,
}

#[derive(Debug, Clone, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Catalog {
    SWRC_CATALOG,
    WINSLOW_MESSENGER_ICME_CATALOG,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpsParams {
    /// Default: 30 days prior to current UTC date
    start_date: Option<String>,
    /// Default: current UTC date
    end_date: Option<String>,
    /// Default: ALL
    location: Option<Location>,
    /// Default: ALL
    catalog: Option<Catalog>,
}

impl crate::query::QueryValues for IpsParams {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut values = std::collections::HashMap::new();
        if let Some(start_date) = &self.start_date {
            values.insert("startDate".to_string(), start_date.to_string());
        }
        if let Some(end_date) = &self.end_date {
            values.insert("endDate".to_string(), end_date.to_string());
        }
        if let Some(location) = &self.location {
            values.insert(
                "location".to_string(),
                serde_json::to_string(location).unwrap(),
            );
        }
        if let Some(catalog) = &self.catalog {
            values.insert(
                "catalog".to_string(),
                serde_json::to_string(catalog).unwrap(),
            );
        }
        values
    }
}
