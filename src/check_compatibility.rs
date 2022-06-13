use serde::{Deserialize, Serialize};
use crate::types::{Dimension, DimensionMetadata, FilterExpression, Metric, MetricMetadata};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CheckCompatibilityRequest {
    dimensions: Option<Vec<Dimension>>,
    metrics: Option<Vec<Metric>>,
    #[serde(rename = "dimensionFilter")]
    dimension_filter: Option<FilterExpression>,
    #[serde(rename = "metricFilter")]
    metric_filter: Option<FilterExpression>,
    #[serde(rename = "compatibilityFilter")]
    compatibility_filter: Option<Compatibility>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CheckCompatibilityResponse {
    #[serde(rename = "dimensionCompatibilities")]
    dimension_compatibilities: Option<Vec<DimensionCompatibility>>,
    #[serde(rename = "metricCompatibilities")]
    metric_compatibilities: Option<Vec<MetricCompatibility>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseCheckCompatibility {
    #[serde(rename = "dimensionCompatibilities")]
    dimension_compatibilities: Option<Vec<DimensionCompatibility>>,
    #[serde(rename = "metricCompatibilities")]
    metric_compatibilities: Option<Vec<MetricCompatibility>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionCompatibility {
    #[serde(rename = "dimensionMetadata")]
    dimension_metadata: Option<DimensionMetadata>,
    compatibility: Option<Compatibility>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Compatibility {
    #[serde(rename = "COMPATIBILITY_UNSPECIFIED")]
    CompatibilityUnspecified,
    #[serde(rename = "COMPATIBLE")]
    Compatible,
    #[serde(rename = "INCOMPATIBLE")]
    Incompatible,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MetricCompatibility {
    #[serde(rename = "metric_metadata")]
    metric_metadata: Option<MetricMetadata>,
    compatibility: Option<String>,

}

