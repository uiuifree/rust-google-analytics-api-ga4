#![doc = include_str!("../README.md")]

mod batch_run_pivot_reports;
mod batch_run_reports;
mod check_compatibility;
mod error;
mod get_metadata;
mod http;
mod run_pivot_reports;
mod run_realtime_report;
mod run_report;
pub mod types;

pub use batch_run_pivot_reports::BatchRunPivotReportsResponse;
pub use batch_run_reports::BatchRunReportsResponse;
pub use check_compatibility::*;
pub use error::*;
pub use get_metadata::*;
pub use run_pivot_reports::*;
pub use run_realtime_report::*;
pub use run_report::*;

use crate::batch_run_pivot_reports::BatchRunPivotReportsRequestBody;
use crate::batch_run_reports::BatchRunReportsRequestBody;
use crate::http::HttpClient;
use crate::types::{RunPivotReportResponse, RunReportResponse};

const ENDPOINT: &str = "https://analyticsdata.googleapis.com/v1beta";

/// Entry point for the Google Analytics 4 (GA4) Data API v1beta.
///
/// Each method takes an OAuth2 access token (`&str`) and a numeric GA4
/// property ID (e.g. `"123456789"`).
pub struct AnalyticsDataApi {}

impl AnalyticsDataApi {
    /// Runs multiple pivot reports in a single API request.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunPivotReports>
    pub async fn batch_run_pivot_reports(
        token: &str,
        property: &str,
        requests: Vec<RunPivotReportRequest>,
    ) -> Result<BatchRunPivotReportsResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:batchRunPivotReports", ENDPOINT, property).as_str(),
            BatchRunPivotReportsRequestBody { requests },
        )
        .await
    }
    /// Runs up to 5 reports in a single API request.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunReports>
    pub async fn batch_run_reports(
        token: &str,
        property: &str,
        requests: Vec<RunReportRequest>,
    ) -> Result<BatchRunReportsResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:batchRunReports", ENDPOINT, property).as_str(),
            BatchRunReportsRequestBody { requests },
        )
        .await
    }
    /// Checks which dimensions and metrics can be combined in a report.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/checkCompatibility>
    pub async fn check_compatibility(
        token: &str,
        property: &str,
        request: CheckCompatibilityRequest,
    ) -> Result<CheckCompatibilityResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:checkCompatibility", ENDPOINT, property).as_str(),
            request,
        )
        .await
    }
    /// Lists all dimensions and metrics available on a property, including custom definitions.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/getMetadata>
    pub async fn get_metadata(
        token: &str,
        property: &str,
    ) -> Result<GetMetadataResponse, GoogleApiError> {
        HttpClient::get(
            token,
            format!("{}/properties/{}/metadata", ENDPOINT, property).as_str(),
        )
        .await
    }
    /// Runs a pivot-table style report.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runPivotReport>
    pub async fn run_pivot_report(
        token: &str,
        property: &str,
        request: RunPivotReportRequest,
    ) -> Result<RunPivotReportResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:runPivotReport", ENDPOINT, property).as_str(),
            request,
        )
        .await
    }
    /// Returns realtime event data for the last 30 minutes (e.g. active users).
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runRealtimeReport>
    pub async fn run_realtime_report(
        token: &str,
        property: &str,
        request: RunRealtimeReportRequest,
    ) -> Result<RunReportResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:runRealtimeReport", ENDPOINT, property).as_str(),
            request,
        )
        .await
    }
    /// Runs a report of dimensions and metrics over a date range.
    ///
    /// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runReport>
    pub async fn run_report(
        token: &str,
        property: &str,
        request: RunReportRequest,
    ) -> Result<RunReportResponse, GoogleApiError> {
        HttpClient::post(
            token,
            format!("{}/properties/{}:runReport", ENDPOINT, property).as_str(),
            request,
        )
        .await
    }
}
