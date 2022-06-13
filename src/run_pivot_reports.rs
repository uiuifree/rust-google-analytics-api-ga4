use crate::types::{CohortSpec, DateRange, Dimension, FilterExpression, Metric, Pivot};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunPivotReportRequest {
    dimensions: Option<Dimension>,
    metrics: Option<Metric>,
    #[serde(rename = "dateRanges")]
    date_ranges: Option<DateRange>,
    pivots: Option<Pivot>,
    #[serde(rename = "dimensionFilter")]
    dimension_filter: Option<FilterExpression>,
    #[serde(rename = "metricFilter")]
    metric_filter: Option<FilterExpression>,
    #[serde(rename = "currencyCode")]
    currency_code: Option<String>,
    #[serde(rename = "cohortSpec")]
    cohort_spec: Option<CohortSpec>,
    #[serde(rename = "keepEmptyRows")]
    keep_empty_rows: Option<bool>,
    #[serde(rename = "returnPropertyQuota")]
    return_property_quota: Option<bool>,
}

