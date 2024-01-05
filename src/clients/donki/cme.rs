#![allow(non_camel_case_types)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Catalog {
    ALL,
    SWRC_CATALOG,
    JANG_ET_AL_CATALOG,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CmeAnalysisParams {
    /// default 30 days prior to current UTC time
    start_date: Option<String>,
    /// default is set to current UTC time
    end_date: Option<String>,
    /// default is set to true
    most_accurate_only: Option<bool>,
    /// default is set to true
    complete_entry_only: Option<bool>,
    /// (lower limit) default is set to 0
    speed: Option<f64>,
    /// (lower limit) default is set to 0
    half_angle: Option<f64>,
    /// default is set to ALL; see [Catalog](enum.Catalog.html) for more info
    catalog: Option<Catalog>,
    /// default is set to NONE (example choices: swpc_annex)
    keyword: Option<String>,
}

impl Default for CmeAnalysisParams {
    fn default() -> Self {
        CmeAnalysisParams {
            start_date: None,
            end_date: None,
            most_accurate_only: None,
            complete_entry_only: None,
            speed: None,
            half_angle: None,
            catalog: None,
            keyword: None,
        }
    }
}

impl QueryValues for CmeAnalysisParams {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(start_date) = &self.start_date {
            map.insert("startDate".to_string(), start_date.to_string());
        }
        if let Some(end_date) = &self.end_date {
            map.insert("endDate".to_string(), end_date.to_string());
        }
        if let Some(most_accurate_only) = self.most_accurate_only {
            map.insert(
                "mostAccurateOnly".to_string(),
                most_accurate_only.to_string(),
            );
        }
        if let Some(complete_entry_only) = self.complete_entry_only {
            map.insert(
                "completeEntryOnly".to_string(),
                complete_entry_only.to_string(),
            );
        }
        if let Some(speed) = self.speed {
            map.insert("speed".to_string(), speed.to_string());
        }
        if let Some(half_angle) = self.half_angle {
            map.insert("halfAngle".to_string(), half_angle.to_string());
        }
        if let Some(catalog) = &self.catalog {
            map.insert("catalog".to_string(), format!("{:?}", catalog));
        }
        if let Some(keyword) = &self.keyword {
            map.insert("keyword".to_string(), keyword.to_owned());
        }
        map
    }
}

/// CME (Coronal Mass Ejection) is a large release of plasma and magnetic field from the solar corona.
pub struct CmeAnalysis;

impl Spec for CmeAnalysis {
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/CME";
    type Params = CmeAnalysisParams;


    fn build_query(params: &Self::Params) -> String {
        todo!()
    }

    fn parse_response(res: reqwest::blocking::Response) -> Self::ResponseType {
        res.json().unwrap()
    }
}
