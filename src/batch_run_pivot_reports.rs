use crate::run_pivot_reports::RunPivotReportRequest;
use crate::types::RunPivotReportResponse;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct BatchRunPivotReportsRequestBody {
    pub requests: Vec<RunPivotReportRequest>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunPivotReports>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchRunPivotReportsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_reports: Option<Vec<RunPivotReportResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
