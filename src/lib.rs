mod error;
mod http;
mod check_compatibility;
mod run_report;
pub mod types;
mod run_realtime_report;
mod get_metadata;
mod batch_run_pivot_reports;
mod batch_run_reports;
mod run_pivot_reports;

use serde_json::json;
use crate::{GoogleApiError};
pub use run_report::*;
pub use error::*;
pub use check_compatibility::*;
pub use run_realtime_report::*;
pub use get_metadata::*;
use crate::batch_run_pivot_reports::{BatchRunPivotReportRequest, BatchRunPivotReportRequestBody, BatchRunPivotReportResponse};
use crate::batch_run_reports::{BatchRunReportRequestBody};
use crate::http::HttpClient;
use crate::types::RunReportResponse;


pub struct AnalyticsDataApi {}

impl AnalyticsDataApi {
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunPivotReports
    pub async fn batch_run_pivot_reports(token: &str, property: &str, requests: Vec<BatchRunPivotReportRequest>) -> Result<BatchRunPivotReportResponse, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:batchRunPivotReports"#, property, ).as_str(),
            BatchRunPivotReportRequestBody {
                requests
            },
        ).await?)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunReports
    pub async fn batch_run_reports(token: &str, property: &str, requests: Vec<RunReportRequest>) -> Result<RunReportResponse, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:batchRunReports"#, property, ).as_str(),
            BatchRunReportRequestBody {
                requests
            },
        ).await?)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/checkCompatibility
    pub async fn check_compatibility(token: &str, property: &str, request: CheckCompatibilityRequest) -> Result<CheckCompatibilityResponse, GoogleApiError> {
        Ok(HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:checkCompatibility"#, property, ).as_str(),
            request,
        ).await?)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/getMetadata
    pub async fn get_metadata(token: &str, property: &str) -> Result<GetMetadataResponse, GoogleApiError> {
        let data = HttpClient::get(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:metadata"#, property).as_str(),
            json!({}),
        ).await?;
        Ok(data)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runPivotReport
    pub async fn run_pivot_report(token: &str, property: &str, request: CheckCompatibilityRequest) -> Result<CheckCompatibilityResponse, GoogleApiError> {
        let data = HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:runPivotReport"#, property).as_str(),
            request,
        ).await?;
        Ok(data)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runRealtimeReport
    pub async fn run_realtime_report(token: &str, property: &str, request: RunRealtimeReportRequest) -> Result<RunReportResponse, GoogleApiError> {
        let data = HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:runRealtimeReport"#, property).as_str(),
            request,
        ).await?;
        Ok(data)
    }
    /// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runReport
    pub async fn run_report(token: &str, property: &str, request: RunReportRequest) -> Result<RunReportResponse, GoogleApiError> {
        let data = HttpClient::post(
            token,
            format!(r#"https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport"#, property).as_str(),
            request,
        ).await?;
        Ok(data)
    }
}


pub struct BatchRunReportsRequestBody {}


