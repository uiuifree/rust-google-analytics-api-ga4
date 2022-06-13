use serde::{Deserialize, Serialize};
use crate::RunReportRequest;

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct BatchRunReportRequestBody {
    pub requests: Vec<RunReportRequest>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunReportResponse {
    #[serde(rename = "pivotReports")]
    reports: Option<Vec<RunReportResponse>>,
    kind: Option<String>,
}