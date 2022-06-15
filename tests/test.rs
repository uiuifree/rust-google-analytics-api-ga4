use yup_oauth2::{AccessToken};
use google_analytics_api_ga4::{AnalyticsDataApi, RunRealtimeReportRequest, RunReportRequest};
use google_analytics_api_ga4::types::{DateRange, Dimension, Filter, FilterExpression, FilterExpressionList, MatchType, Metric, StringFilter};


async fn test_token() -> AccessToken {
    // 認証
    let secret = yup_oauth2::read_service_account_key("./test.json")
        .await
        .expect("test.json");
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret).build().await.unwrap();
    let scopes = &["https://www.googleapis.com/auth/analytics.readonly"];

    let token = auth.token(scopes).await;
    assert!(token.is_ok(), "{}", token.err().unwrap().to_string());
    token.unwrap()
}


#[tokio::test]
async fn test_run_realtime_report() {
    let token = test_token().await;
    let property_id = "252735638";

    let metric_values = vec![
        "activeUsers",
    ];
    let dimension_values = vec![
        "appVersion",
    ];

    let mut metrics = vec![];
    let mut dimensions = vec![];
    for value in metric_values {
        metrics.push(Metric::new(value));
    }
    for value in dimension_values {
        dimensions.push(Dimension::new(value));
    }


    let a = RunRealtimeReportRequest {
        dimensions,
        metrics,
        limit: Some("2".to_string()),
        ..RunRealtimeReportRequest::default()
    };


    let run_report = AnalyticsDataApi::run_realtime_report(token.as_str(), property_id, a).await;
    assert!(run_report.is_ok(), "{}", run_report.err().unwrap().to_string());
    let run_report = run_report.unwrap();
    let metric_headers = run_report.metric_headers.unwrap_or_default();
    let rows = run_report.rows.unwrap_or_default();
    for header in metric_headers {
        println!("{:?}", header);
    }
    for row in rows {
        println!("{:?}", row.metric_values);
    }
}


#[tokio::test]
async fn test_run_report() {
    let token = test_token().await;
    let property_id = "252735638";

    let metric_values = vec![
        "sessions",
        "screenPageViews",
        "eventCount",
        "eventValue",
    ];
    let dimension_values = vec![
        "fullPageUrl",
        "eventName",
    ];

    let mut metrics = vec![];
    let mut dimensions = vec![];
    for value in metric_values {
        metrics.push(Metric::new(value));
    }
    for value in dimension_values {
        dimensions.push(Dimension::new(value));
    }

    let mut filter_expression = FilterExpression::default();

    let mut filter_list = FilterExpressionList::default();
    // filter_list.expressions.

    let mut filters = vec![];
    filters.push(FilterExpression {
        filter: Some(
            Filter {
                field_name: Some("fullPageUrl".to_string()),
                string_filter: Some(StringFilter {
                    match_type: Some(MatchType::BeginsWith),
                    value: Some("uiuifree.com".to_string()),
                    ..StringFilter::default()
                }),
                ..Filter::default()
            }
        ),
        ..FilterExpression::default()
    });
    filters.push(FilterExpression {
        filter: Some(
            Filter {
                field_name: Some("eventName".to_string()),
                string_filter: Some(StringFilter {
                    match_type: Some(MatchType::BeginsWith),
                    value: Some("session_start".to_string()),
                    ..StringFilter::default()
                }),
                ..Filter::default()
            }
        ),
        ..FilterExpression::default()
    });
    filter_list.expressions = Some(filters);


    filter_expression.and_group = Some(filter_list);
    let a = RunReportRequest {
        property: format!("properties/{}", property_id.to_string()),
        dimensions,
        metrics,
        date_ranges: vec![DateRange::new("test", "2022-01-01", "2022-06-12")],
        dimension_filter: Some(filter_expression),
        limit: Some("20".to_string()),
        ..RunReportRequest::default()
    };


    let run_report = AnalyticsDataApi::run_report(token.as_str(), property_id, a).await;
    assert!(run_report.is_ok(), "{}", run_report.err().unwrap().to_string());
    let run_report = run_report.unwrap();
    let metric_headers = run_report.metric_headers.unwrap_or_default();
    // let dimension_headers = run_report.dimension_headers.unwrap_or_default();
    let rows = run_report.rows.unwrap_or_default();
    for header in metric_headers {
        println!("{:?}", header);
    }
    for row in rows {
        println!("{:?}", row.metric_values);
        println!("{:?}", row.dimension_values);
    }
}


