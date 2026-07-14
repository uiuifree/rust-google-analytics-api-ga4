use serde::{Deserialize, Serialize};

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/CohortSpec>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohortSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohorts: Option<Vec<Cohort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohorts_range: Option<CohortsRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_report_settings: Option<CohortReportSettings>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cohort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohortsRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/CohortSpec#Granularity>
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Granularity {
    GranularityUnspecified,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CohortReportSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate: Option<bool>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DateRange>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DateRange {
    pub fn new(name: &str, start: &str, end: &str) -> DateRange {
        DateRange {
            name: Some(name.to_string()),
            start_date: Some(start.to_string()),
            end_date: Some(end.to_string()),
        }
    }
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Dimension>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_expression: Option<DimensionExpression>,
}

impl Dimension {
    pub fn new(name: &str) -> Dimension {
        Dimension {
            name: Some(name.to_string()),
            ..Dimension::default()
        }
    }
    pub fn from_string_vec(vals: Vec<&str>) -> Vec<Self> {
        vals.into_iter().map(Self::new).collect()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_case: Option<CaseExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_case: Option<CaseExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concatenate: Option<ConcatenateExpression>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CaseExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConcatenateExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionHeader>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionMetadata>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_api_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_definition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionValue>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/FilterExpression>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_group: Option<FilterExpressionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_group: Option<FilterExpressionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_expression: Option<Box<FilterExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterExpressionList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressions: Option<Vec<FilterExpression>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filter: Option<StringFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_list_filter: Option<InListFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_filter: Option<NumericFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub between_filter: Option<BetweenFilter>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<MatchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchType {
    MatchTypeUnspecified,
    Exact,
    BeginsWith,
    EndsWith,
    Contains,
    FullRegexp,
    PartialRegexp,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InListFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumericFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<NumericValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    OperationUnspecified,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumericValue {
    #[serde(rename = "int64Value", skip_serializing_if = "Option::is_none")]
    pub int64_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BetweenFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_value: Option<NumericValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_value: Option<NumericValue>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Metric>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible: Option<bool>,
}

impl Metric {
    pub fn new(name: &str) -> Metric {
        Metric {
            name: Some(name.to_string()),
            ..Metric::default()
        }
    }
    pub fn from_string_vec(vals: Vec<&str>) -> Vec<Self> {
        vals.into_iter().map(Self::new).collect()
    }
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricAggregation>
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricAggregation {
    MetricAggregationUnspecified,
    Total,
    Minimum,
    Maximum,
    Count,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricHeader>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricMetadata>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_api_names: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_definition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reasons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricType>
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricType {
    MetricTypeUnspecified,
    TypeInteger,
    TypeFloat,
    TypeSeconds,
    TypeMilliseconds,
    TypeMinutes,
    TypeHours,
    TypeStandard,
    TypeCurrency,
    TypeFeet,
    TypeMiles,
    TypeMeters,
    TypeKilometers,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/OrderBy>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<MetricOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<DimensionOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot: Option<PivotOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

impl OrderBy {
    pub fn metric(metric_name: &str, desc: bool) -> OrderBy {
        OrderBy {
            metric: Some(MetricOrderBy {
                metric_name: Some(metric_name.to_string()),
            }),
            desc: Some(desc),
            ..OrderBy::default()
        }
    }
    pub fn dimension(dimension_name: &str, desc: bool) -> OrderBy {
        OrderBy {
            dimension: Some(DimensionOrderBy {
                dimension_name: Some(dimension_name.to_string()),
                order_type: None,
            }),
            desc: Some(desc),
            ..OrderBy::default()
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricOrderBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionOrderBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    OrderTypeUnspecified,
    Alphanumeric,
    CaseInsensitiveAlphanumeric,
    Numeric,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PivotOrderBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_selections: Option<Vec<PivotSelection>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PivotSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Pivot>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pivot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_bys: Option<Vec<OrderBy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/PropertyQuota>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyQuota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_day: Option<QuotaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_hour: Option<QuotaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_requests: Option<QuotaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_errors_per_project_per_hour: Option<QuotaStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potentially_thresholded_requests_per_hour: Option<QuotaStatus>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuotaStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/ResponseMetaData>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loss_from_other_row: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_restriction_response: Option<SchemaRestrictionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_to_thresholding: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRestrictionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_metric_restrictions: Option<Vec<ActiveMetricRestriction>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMetricRestriction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_metric_types: Option<Vec<RestrictedMetricType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RestrictedMetricType {
    RestrictedMetricTypeUnspecified,
    CostData,
    RevenueData,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Row>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_values: Option<Vec<DimensionValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_values: Option<Vec<MetricValue>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunPivotReportResponse>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunPivotReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_headers: Option<Vec<PivotHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResponseMetaData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_quota: Option<PropertyQuota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PivotHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_dimension_headers: Option<Vec<PivotDimensionHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PivotDimensionHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_values: Option<Vec<DimensionValue>>,
}

/// <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunReportResponse>
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximums: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimums: Option<Vec<Row>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResponseMetaData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_quota: Option<PropertyQuota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
