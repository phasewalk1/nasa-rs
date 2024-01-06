use serde::Serialize;

pub use crate::clients::{
    apod::ApodParams,
    donki::{
        cme::CmeAnalysisParams,
        // ...
        // flr,gst,hss,mpc,rbe,sep,wsa all use ParamsCommon
        ips::IpsParams,
    },
    earth::imagery::ParamsEarthImagery,
    neo::NeoParams,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamsCommon {
    /// default to 30 days prior to current UTC date
    start_date: Option<String>,
    /// default to current UTC date
    end_date: Option<String>,
}

impl Default for ParamsCommon {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
        }
    }
}

impl crate::query::QueryValues for ParamsCommon {
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
