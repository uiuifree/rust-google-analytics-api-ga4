use serde::{Deserialize, Serialize};
use crate::types::{ Dimension, DimensionOrderBy, FilterExpression, Metric, MetricAggregation};

/// Realtime Dimensions & Metrics
/// https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-api-schema

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunRealtimeReportRequest {
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<Metric>,
    #[serde(rename = "dimensionFilter")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(rename = "metricFilter")]
    pub metric_filter: Option<FilterExpression>,
    pub limit: Option<String>,
    #[serde(rename = "metricAggregations")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
    #[serde(rename = "orderBys")]
    pub order_bys: Option<Vec<DimensionOrderBy>>,
    #[serde(rename = "returnPropertyQuota")]
    pub return_property_quota: Option<bool>,
    #[serde(rename = "minuteRanges")]
    pub minute_ranges: Option<Vec<MinuteRange>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MinuteRange {
    name: Option<String>,
    #[serde(rename = "startMinutesAgo")]
    start_minutes_ago: Option<String>,
    #[serde(rename = "endMinutesAgo")]
    end_minutes_ago: Option<String>,
}