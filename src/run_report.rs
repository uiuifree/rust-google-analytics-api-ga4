use crate::types::{
    CohortSpec, DateRange, Dimension, FilterExpression, Metric, MetricAggregation, OrderBy,
};
use serde::{Deserialize, Serialize};

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runReport>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunReportRequest {
    pub property: String,
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<Metric>,
    pub date_ranges: Vec<DateRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_bys: Option<Vec<OrderBy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_spec: Option<CohortSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_empty_rows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_property_quota: Option<bool>,
}
