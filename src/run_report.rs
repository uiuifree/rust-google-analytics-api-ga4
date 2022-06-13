use serde::{Deserialize, Serialize};
use crate::types::{CohortSpec, DateRange, Dimension, DimensionOrderBy, FilterExpression, Metric, MetricAggregation};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunReportRequest {
    pub property: String,
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<Metric>,
    #[serde(rename = "dateRanges")]
    pub date_ranges: Vec<DateRange>,
    #[serde(rename = "dimensionFilter")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(rename = "metricFilter")]
    pub metric_filter: Option<FilterExpression>,
    pub offset: Option<String>,
    pub limit: Option<String>,
    #[serde(rename = "metricAggregations")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
    #[serde(rename = "orderBys")]
    pub order_bys: Option<Vec<DimensionOrderBy>>,
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "cohortSpec")]
    pub cohort_spec: Option<CohortSpec>,
    #[serde(rename = "keepEmptyRows")]
    pub keep_empty_rows: Option<bool>,
    #[serde(rename = "returnPropertyQuota")]
    pub return_property_quota: Option<bool>,
}