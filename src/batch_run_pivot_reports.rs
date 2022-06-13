use crate::types::{CohortSpec, DateRange, Dimension, FilterExpression, Metric, Pivot, RunPivotReportResponse};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct BatchRunPivotReportRequestBody {
    pub requests: Vec<BatchRunPivotReportRequest>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportRequest {
    property: Option<String>,
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportResponse {
    #[serde(rename = "pivotReports")]
    pivot_reports: Option<Vec<RunPivotReportResponse>>,
    kind: Option<String>,
}