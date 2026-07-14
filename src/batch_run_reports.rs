use crate::types::RunReportResponse;
use crate::RunReportRequest;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct BatchRunReportsRequestBody {
    pub requests: Vec<RunReportRequest>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunReports>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchRunReportsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<RunReportResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
