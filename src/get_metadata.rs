use crate::types::{DimensionMetadata,  MetricMetadata};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetMetadataResponse {
    name: Option<String>,
    dimensions: Option<Vec<DimensionMetadata>>,
    metrics: Option<Vec<MetricMetadata>>,
}