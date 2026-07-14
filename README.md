# google-analytics-api-ga4 — Google Analytics 4 (GA4) Data API client for Rust

[![crates.io](https://img.shields.io/crates/v/google-analytics-api-ga4.svg)](https://crates.io/crates/google-analytics-api-ga4)
[![docs.rs](https://img.shields.io/docsrs/google-analytics-api-ga4)](https://docs.rs/google-analytics-api-ga4)
[![downloads](https://img.shields.io/crates/d/google-analytics-api-ga4.svg)](https://crates.io/crates/google-analytics-api-ga4)
[![license](https://img.shields.io/crates/l/google-analytics-api-ga4.svg)](https://github.com/uiuifree/rust-google-analytics-api-ga4/blob/main/LICENSE)

**google-analytics-api-ga4** is a lightweight, async Rust client library for the
[Google Analytics Data API v1beta (GA4)](https://developers.google.com/analytics/devguides/reporting/data/v1/rest).
Fetch GA4 reports, realtime reports, pivot reports, and metadata from Rust with
strongly typed requests and responses — no code generation, no heavy dependencies
(just `reqwest`, `serde`, and `serde_json`).

Use it to build analytics dashboards, export GA4 data to your database or data
warehouse, automate SEO / marketing reporting, or monitor realtime active users
from a Rust backend.

## Features

| GA4 Data API method | Rust function | Description |
|---|---|---|
| [`runReport`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runReport) | `AnalyticsDataApi::run_report` | Core report of dimensions and metrics (sessions, pageviews, events, …) |
| [`batchRunReports`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunReports) | `AnalyticsDataApi::batch_run_reports` | Up to 5 reports in a single HTTP request |
| [`runPivotReport`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runPivotReport) | `AnalyticsDataApi::run_pivot_report` | Pivot-table style reports |
| [`batchRunPivotReports`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/batchRunPivotReports) | `AnalyticsDataApi::batch_run_pivot_reports` | Multiple pivot reports in one request |
| [`runRealtimeReport`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runRealtimeReport) | `AnalyticsDataApi::run_realtime_report` | Realtime active users for the last 30 minutes |
| [`checkCompatibility`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/checkCompatibility) | `AnalyticsDataApi::check_compatibility` | Check which dimensions/metrics can be combined |
| [`getMetadata`](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/getMetadata) | `AnalyticsDataApi::get_metadata` | List all available dimensions and metrics, including custom ones |

- **Async / await** — built on `reqwest` + `tokio`, with a shared connection pool
- **Strongly typed** — request and response structs mirror the official GA4 REST schema (`FilterExpression`, `OrderBy`, `CohortSpec`, `Pivot`, …)
- **Filter support** — string / in-list / numeric / between filters with `andGroup`, `orGroup`, `notExpression`
- **Proper error handling** — `GoogleApiError` implements `std::error::Error`, distinguishing network errors, non-2xx API responses, and JSON parse failures
- **Auth-agnostic** — pass any OAuth2 / service-account access token as `&str` (works with `yup-oauth2`, `gcp_auth`, etc.)

## Installation

```toml
[dependencies]
google-analytics-api-ga4 = "0.2"
```

## Quick start

### 1. Get an access token (service account)

Any library that produces a Google OAuth2 access token works. Example with
[yup-oauth2](https://crates.io/crates/yup-oauth2):

```rust
async fn token() -> yup_oauth2::AccessToken {
    let secret = yup_oauth2::read_service_account_key("./service_account.json")
        .await
        .expect("service_account.json");
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .unwrap();
    let scopes = &["https://www.googleapis.com/auth/analytics.readonly"];
    auth.token(scopes).await.unwrap()
}
```

> Remember to add the service account's email address as a viewer of your GA4
> property (Admin → Property Access Management).

### 2. Run a report (`runReport`)

```rust
use google_analytics_api_ga4::types::{
    DateRange, Dimension, Filter, FilterExpression, FilterExpressionList, MatchType, Metric,
    OrderBy, StringFilter,
};
use google_analytics_api_ga4::{AnalyticsDataApi, RunReportRequest};

async fn example(token: &str) {
    let property_id = "123456789";

    // WHERE fullPageUrl BEGINS_WITH "example.com"
    let dimension_filter = FilterExpression {
        and_group: Some(FilterExpressionList {
            expressions: Some(vec![FilterExpression {
                filter: Some(Filter {
                    field_name: Some("fullPageUrl".to_string()),
                    string_filter: Some(StringFilter {
                        match_type: Some(MatchType::BeginsWith),
                        value: Some("example.com".to_string()),
                        ..StringFilter::default()
                    }),
                    ..Filter::default()
                }),
                ..FilterExpression::default()
            }]),
        }),
        ..FilterExpression::default()
    };

    let request = RunReportRequest {
        property: format!("properties/{}", property_id),
        dimensions: Dimension::from_string_vec(vec!["fullPageUrl", "eventName"]),
        metrics: Metric::from_string_vec(vec!["sessions", "screenPageViews", "eventCount"]),
        date_ranges: vec![DateRange::new("range", "2024-01-01", "2024-01-31")],
        dimension_filter: Some(dimension_filter),
        order_bys: Some(vec![OrderBy::metric("sessions", true)]), // ORDER BY sessions DESC
        limit: Some("20".to_string()),
        ..RunReportRequest::default()
    };

    let response = AnalyticsDataApi::run_report(token, property_id, request)
        .await
        .unwrap();
    for row in response.rows.unwrap_or_default() {
        println!("{:?} {:?}", row.dimension_values, row.metric_values);
    }
}
```

### 3. Realtime active users (`runRealtimeReport`)

```rust
use google_analytics_api_ga4::types::{Dimension, Metric};
use google_analytics_api_ga4::{AnalyticsDataApi, MinuteRange, RunRealtimeReportRequest};

async fn example(token: &str) {
    let property_id = "123456789";

    let request = RunRealtimeReportRequest {
        dimensions: Dimension::from_string_vec(vec!["unifiedScreenName"]),
        metrics: Metric::from_string_vec(vec!["activeUsers"]),
        minute_ranges: Some(vec![MinuteRange::new("last 5 min", 5, 0)]),
        ..RunRealtimeReportRequest::default()
    };

    let response = AnalyticsDataApi::run_realtime_report(token, property_id, request)
        .await
        .unwrap();
    println!("{:?}", response.rows);
}
```

### 4. Discover dimensions and metrics (`getMetadata`)

```rust
use google_analytics_api_ga4::AnalyticsDataApi;

async fn example(token: &str) {
    let metadata = AnalyticsDataApi::get_metadata(token, "123456789")
        .await
        .unwrap();
    for dimension in metadata.dimensions.unwrap_or_default() {
        println!("{:?}: {:?}", dimension.api_name, dimension.ui_name);
    }
}
```

## Error handling

Every method returns `Result<_, GoogleApiError>`:

| Variant | Meaning |
|---|---|
| `GoogleApiError::Connection(message)` | Network-level failure (DNS, TLS, timeout) |
| `GoogleApiError::Response(status, body)` | The API returned a non-2xx status (quota exceeded, permission denied, invalid argument, …) |
| `GoogleApiError::JsonParse(body)` | The response body could not be deserialized |

`GoogleApiError` implements `std::error::Error` and `Display`, so it composes
with `?`, `anyhow`, and `thiserror`.

## FAQ

**Q. Does this crate support Universal Analytics (UA / GA3)?**
No. It targets Google Analytics 4 (GA4) only, via the Data API v1beta. Universal
Analytics was shut down by Google in 2023–2024.

**Q. Which authentication methods are supported?**
Any Google OAuth2 access token: service accounts (recommended for servers),
user OAuth flows, workload identity, etc. The token is passed as a plain `&str`,
so you can use `yup-oauth2`, `gcp_auth`, or your own token source.

**Q. How is this different from `google-analyticsdata1_beta` (google-apis-rs)?**
This crate is hand-written and minimal: a handful of files, three dependencies,
and idiomatic builder-free structs. The auto-generated alternative covers more
surface area but is heavier and harder to read.

**Q. Which GA4 dimensions and metrics can I use?**
Any from the official
[API schema](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema)
(e.g. `sessions`, `activeUsers`, `screenPageViews`, `eventCount`, `country`,
`fullPageUrl`, `eventName`), plus your custom definitions. Use
`AnalyticsDataApi::get_metadata` to list everything available on your property,
and `check_compatibility` to verify combinations.

**Q. Is the Data API quota handled?**
Set `return_property_quota: Some(true)` on a request and read
`response.property_quota` to monitor consumed/remaining tokens per property.

## 日本語での概要

**google-analytics-api-ga4** は、Google アナリティクス 4（GA4）の Data API v1beta を
Rust から呼び出すための非同期クライアントライブラリです。`runReport` による
レポート取得、`runRealtimeReport` によるリアルタイムのアクティブユーザー数取得、
ピボットレポート、ディメンション・指標のメタデータ取得に対応しています。
サービスアカウントのアクセストークンを文字列で渡すだけで利用でき、GA4 データの
データベース連携・ダッシュボード構築・SEO レポート自動化などに使えます。

## Testing

```shell
# Offline serialization tests (no credentials needed)
cargo test

# Integration tests against the live API (requires ./test.json service account key)
cargo test -- --ignored
```

## Links

- [API reference on docs.rs](https://docs.rs/google-analytics-api-ga4)
- [crates.io page](https://crates.io/crates/google-analytics-api-ga4)
- [Google Analytics Data API v1beta reference](https://developers.google.com/analytics/devguides/reporting/data/v1/rest)
- [GA4 dimensions & metrics schema](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema)
- [Realtime API schema](https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-api-schema)

## Breaking changes in 0.2.0

- `AnalyticsDataApi::run_pivot_report` now takes `RunPivotReportRequest` and returns `RunPivotReportResponse` (previously it mistakenly used the checkCompatibility types).
- `AnalyticsDataApi::batch_run_reports` now returns `BatchRunReportsResponse` (with a `reports` field).
- `AnalyticsDataApi::batch_run_pivot_reports` now takes `Vec<RunPivotReportRequest>` and returns `BatchRunPivotReportsResponse`.
- `RunReportRequest::order_bys` / `RunRealtimeReportRequest::order_bys` are now `Option<Vec<OrderBy>>` (matching the API schema; previously `DimensionOrderBy` was serialized without the `OrderBy` wrapper).
- `MinuteRange::start_minutes_ago` / `end_minutes_ago` are now `Option<i32>`.
- `FilterExpression::not_expression` is now `Option<Box<FilterExpression>>`.
- `NumericValue::double_value` is now `Option<f64>`; `int64_value` was renamed from `int_64_value`.
- `MetricMetadata` fields are now all `Option` (robust deserialization).
- All struct fields are now public, and `None` fields are no longer serialized as `null`.
- `GoogleApiError` gained a `Response(u16, String)` variant for non-2xx responses.

## License

MIT
