use crate::types::{Dimension, FilterExpression, Metric, MetricAggregation, OrderBy};
use serde::{Deserialize, Serialize};

/// Realtime Dimensions & Metrics
/// <https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-api-schema>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunRealtimeReportRequest {
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<Metric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_bys: Option<Vec<OrderBy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_property_quota: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_ranges: Option<Vec<MinuteRange>>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MinuteRange>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinuteRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_minutes_ago: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_minutes_ago: Option<i32>,
}

impl MinuteRange {
    pub fn new(name: &str, start_minutes_ago: i32, end_minutes_ago: i32) -> MinuteRange {
        MinuteRange {
            name: Some(name.to_string()),
            start_minutes_ago: Some(start_minutes_ago),
            end_minutes_ago: Some(end_minutes_ago),
        }
    }
    pub fn single_minute(time: i32) -> MinuteRange {
        MinuteRange::new(format!("Minute {}", time).as_str(), time, time)
    }
}
