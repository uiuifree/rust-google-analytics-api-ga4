use crate::types::{CohortSpec, DateRange, Dimension, FilterExpression, Metric, Pivot};
use serde::{Deserialize, Serialize};

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runPivotReport>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunPivotReportRequest {
    pub property: String,
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<Metric>,
    pub date_ranges: Vec<DateRange>,
    pub pivots: Vec<Pivot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_spec: Option<CohortSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_rows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_property_quota: Option<bool>,
}
