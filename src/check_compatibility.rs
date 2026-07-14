use crate::types::{Dimension, DimensionMetadata, FilterExpression, Metric, MetricMetadata};
use serde::{Deserialize, Serialize};

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/checkCompatibility>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckCompatibilityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter: Option<FilterExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_filter: Option<Compatibility>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckCompatibilityResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_compatibilities: Option<Vec<DimensionCompatibility>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_compatibilities: Option<Vec<MetricCompatibility>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionCompatibility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_metadata: Option<DimensionMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Compatibility>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricCompatibility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_metadata: Option<MetricMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Compatibility>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Compatibility>
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Compatibility {
    CompatibilityUnspecified,
    Compatible,
    Incompatible,
}
