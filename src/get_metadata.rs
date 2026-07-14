use crate::types::{DimensionMetadata, MetricMetadata};
use serde::{Deserialize, Serialize};

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/getMetadata>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<DimensionMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricMetadata>>,
}
