use google_analytics_api_ga4::types::{
    DateRange, Dimension, Filter, FilterExpression, FilterExpressionList, MatchType, Metric,
    OrderBy,
};
use google_analytics_api_ga4::{
    BatchRunPivotReportsResponse, BatchRunReportsResponse, MinuteRange, RunPivotReportRequest,
    RunRealtimeReportRequest, RunReportRequest,
};
use serde_json::json;

#[test]
fn test_run_report_request_serialization() {
    let request = RunReportRequest {
        property: "properties/123".to_string(),
        dimensions: Dimension::from_string_vec(vec!["fullPageUrl", "eventName"]),
        metrics: Metric::from_string_vec(vec!["sessions"]),
        date_ranges: vec![DateRange::new("range", "2024-01-01", "2024-01-31")],
        dimension_filter: Some(FilterExpression {
            and_group: Some(FilterExpressionList {
                expressions: Some(vec![FilterExpression {
                    filter: Some(Filter {
                        field_name: Some("fullPageUrl".to_string()),
                        string_filter: Some(google_analytics_api_ga4::types::StringFilter {
                            match_type: Some(MatchType::BeginsWith),
                            value: Some("example.com".to_string()),
                            case_sensitive: None,
                        }),
                        ..Filter::default()
                    }),
                    ..FilterExpression::default()
                }]),
            }),
            ..FilterExpression::default()
        }),
        order_bys: Some(vec![OrderBy::metric("sessions", true)]),
        limit: Some("20".to_string()),
        ..RunReportRequest::default()
    };

    let actual = serde_json::to_value(&request).unwrap();
    let expected = json!({
        "property": "properties/123",
        "dimensions": [
            {"name": "fullPageUrl"},
            {"name": "eventName"}
        ],
        "metrics": [
            {"name": "sessions"}
        ],
        "dateRanges": [
            {"name": "range", "startDate": "2024-01-01", "endDate": "2024-01-31"}
        ],
        "dimensionFilter": {
            "andGroup": {
                "expressions": [
                    {
                        "filter": {
                            "fieldName": "fullPageUrl",
                            "stringFilter": {
                                "matchType": "BEGINS_WITH",
                                "value": "example.com"
                            }
                        }
                    }
                ]
            }
        },
        "orderBys": [
            {"metric": {"metricName": "sessions"}, "desc": true}
        ],
        "limit": "20"
    });
    assert_eq!(actual, expected);
}

#[test]
fn test_run_realtime_report_request_serialization() {
    let request = RunRealtimeReportRequest {
        dimensions: Dimension::from_string_vec(vec!["appVersion"]),
        metrics: Metric::from_string_vec(vec!["activeUsers"]),
        minute_ranges: Some(vec![MinuteRange::single_minute(5)]),
        limit: Some("2".to_string()),
        ..RunRealtimeReportRequest::default()
    };

    let actual = serde_json::to_value(&request).unwrap();
    let expected = json!({
        "dimensions": [{"name": "appVersion"}],
        "metrics": [{"name": "activeUsers"}],
        "limit": "2",
        "minuteRanges": [
            {"name": "Minute 5", "startMinutesAgo": 5, "endMinutesAgo": 5}
        ]
    });
    assert_eq!(actual, expected);
}

#[test]
fn test_run_pivot_report_request_serialization() {
    let request = RunPivotReportRequest {
        property: "properties/123".to_string(),
        dimensions: Dimension::from_string_vec(vec!["country"]),
        metrics: Metric::from_string_vec(vec!["sessions"]),
        date_ranges: vec![DateRange::new("range", "2024-01-01", "2024-01-31")],
        pivots: vec![google_analytics_api_ga4::types::Pivot {
            field_names: Some(vec!["country".to_string()]),
            limit: Some("5".to_string()),
            ..google_analytics_api_ga4::types::Pivot::default()
        }],
        ..RunPivotReportRequest::default()
    };

    let actual = serde_json::to_value(&request).unwrap();
    let expected = json!({
        "property": "properties/123",
        "dimensions": [{"name": "country"}],
        "metrics": [{"name": "sessions"}],
        "dateRanges": [
            {"name": "range", "startDate": "2024-01-01", "endDate": "2024-01-31"}
        ],
        "pivots": [
            {"fieldNames": ["country"], "limit": "5"}
        ]
    });
    assert_eq!(actual, expected);
}

#[test]
fn test_run_report_response_deserialization() {
    let body = json!({
        "dimensionHeaders": [{"name": "fullPageUrl"}],
        "metricHeaders": [{"name": "sessions", "type": "TYPE_INTEGER"}],
        "rows": [
            {
                "dimensionValues": [{"value": "example.com/"}],
                "metricValues": [{"value": "100"}]
            }
        ],
        "rowCount": 1,
        "metadata": {"currencyCode": "JPY", "timeZone": "Asia/Tokyo"},
        "kind": "analyticsData#runReport"
    })
    .to_string();

    let response: google_analytics_api_ga4::types::RunReportResponse =
        serde_json::from_str(&body).unwrap();
    assert_eq!(response.row_count, Some(1));
    let rows = response.rows.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].metric_values.as_ref().unwrap()[0].value,
        Some("100".to_string())
    );
    assert_eq!(
        response.metric_headers.unwrap()[0].metric_type,
        Some("TYPE_INTEGER".to_string())
    );
}

#[test]
fn test_batch_run_reports_response_deserialization() {
    let body = json!({
        "reports": [
            {"rowCount": 2, "kind": "analyticsData#runReport"}
        ],
        "kind": "analyticsData#batchRunReports"
    })
    .to_string();

    let response: BatchRunReportsResponse = serde_json::from_str(&body).unwrap();
    let reports = response.reports.unwrap();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].row_count, Some(2));
}

#[test]
fn test_batch_run_pivot_reports_response_deserialization() {
    let body = json!({
        "pivotReports": [
            {"kind": "analyticsData#runPivotReport"}
        ],
        "kind": "analyticsData#batchRunPivotReports"
    })
    .to_string();

    let response: BatchRunPivotReportsResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(response.pivot_reports.unwrap().len(), 1);
}

#[test]
fn test_get_metadata_response_deserialization() {
    let body = json!({
        "name": "properties/123/metadata",
        "dimensions": [
            {"apiName": "eventName", "uiName": "Event name", "customDefinition": false}
        ],
        "metrics": [
            {"apiName": "sessions", "uiName": "Sessions", "type": "TYPE_INTEGER"}
        ]
    })
    .to_string();

    let response: google_analytics_api_ga4::GetMetadataResponse =
        serde_json::from_str(&body).unwrap();
    assert_eq!(response.dimensions.unwrap().len(), 1);
    assert_eq!(
        response.metrics.unwrap()[0].metric_type,
        Some("TYPE_INTEGER".to_string())
    );
}

#[test]
fn test_error_display() {
    let error = google_analytics_api_ga4::GoogleApiError::Response(403, "forbidden".to_string());
    assert_eq!(error.to_string(), "api error (status 403): forbidden");
}
