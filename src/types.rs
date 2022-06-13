use serde::{Deserialize, Serialize};

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/CohortSpec#CohortsRange
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CohortSpec {
    cohorts: Option<Vec<Cohort>>,
    #[serde(rename = "cohortsRange")]
    cohorts_range: Option<Vec<Cohort>>,
    #[serde(rename = "cohortReportSettings")]
    cohort_report_settings: Option<CohortReportSettings>,
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Cohort {
    name: Option<String>,
    dimension: Option<String>,
    #[serde(rename = "dateRange")]
    date_range: Option<DateRange>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CohortsRange {
    granularity: Option<String>,
    #[serde(rename = "startOffset")]
    start_offset: Option<i32>,
    #[serde(rename = "endOffset")]
    end_offset: Option<i32>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CohortReportSettings {
    accumulate: Option<bool>,
}
/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DateRange
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
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

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Dimension
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub name: Option<String>,
    #[serde(rename = "dimensionExpression")]
    pub dimension_expression: Option<DimensionExpression>,
}

impl Dimension {
    pub fn new(name: &str) -> Dimension {
        Dimension {
            name: Some(name.to_string()),
            ..Dimension::default()
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionExpression {
    #[serde(rename = "lowerCase")]
    pub lower_case: Option<CaseExpression>,
    #[serde(rename = "upperCase")]
    pub upper_case: Option<CaseExpression>,
    #[serde(rename = "concatenate")]
    pub concatenate: Option<ConcatenateExpression>,

}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CaseExpression {
    #[serde(rename = "dimensionName")]
    pub dimension_name: Option<String>,

}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConcatenateExpression {
    #[serde(rename = "dimensionNames")]
    pub dimension_names: Option<Vec<String>>,
    pub delimiter: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionHeader
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionHeader {
    name: Option<String>,
}


/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionMetadata
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionMetadata {
    #[serde(rename = "apiName")]
    api_name: Option<String>,
    #[serde(rename = "uiName")]
    ui_name: Option<String>,
    description: Option<String>,
    #[serde(rename = "deprecatedApiNames")]
    deprecated_api_names: Option<Vec<String>>,
    #[serde(rename = "customDefinition")]
    custom_definition: Option<bool>,
    category: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionValue
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    pub value: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/FilterExpression
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FilterExpression {
    #[serde(rename = "andGroup")]
    pub and_group: Option<FilterExpressionList>,
    #[serde(rename = "orGroup")]
    pub or_group: Option<FilterExpressionList>,
    #[serde(rename = "notExpression")]
    pub not_expression: Box<Option<FilterExpression>>,
    pub filter: Option<Filter>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FilterExpressionList {
    pub expressions: Option<Vec<FilterExpression>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "fieldName")]
    pub field_name: Option<String>,
    #[serde(rename = "stringFilter")]
    pub string_filter: Option<StringFilter>,
    #[serde(rename = "inListFilter")]
    pub in_list_filter: Option<InListFilter>,
    #[serde(rename = "numericFilter")]
    pub numeric_filter: Option<NumericFilter>,
    #[serde(rename = "betweenFilter")]
    pub between_filter: Option<BetweenFilter>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StringFilter {
    #[serde(rename = "matchType")]
    pub match_type: Option<MatchType>,
    pub value: Option<String>,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchType {
    #[serde(rename = "MATCH_TYPE_UNSPECIFIED")]
    MatchTypeUnspecified,
    #[serde(rename = "EXACT")]
    Exact,
    #[serde(rename = "BEGINS_WITH")]
    BeginsWith,
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "FULL_REGEXP")]
    FullRegexp,
    #[serde(rename = "PARTIAL_REGEXP")]
    PartialRegexp,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InListFilter {
    values: Option<Vec<String>>,
    #[serde(rename = "caseSensitive")]
    case_sensitive: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NumericFilter {
    operation: Option<Operation>,
    value: Option<NumericValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "OPERATION_UNSPECIFIED")]
    OperationUnspecified,
    #[serde(rename = "EQUAL")]
    Equal,
    #[serde(rename = "LESS_THAN")]
    LessThan,
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    LessThanOrEqual,
    #[serde(rename = "GREATER_THAN")]
    GreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    GreaterThanOrEqual,
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NumericValue {
    #[serde(rename = "int64Value")]
    int_64_value: Option<String>,
    #[serde(rename = "doubleValue")]
    double_value: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BetweenFilter {
    #[serde(rename = "fromValue")]
    from_value: Option<NumericValue>,
    #[serde(rename = "toValue")]
    to_value: Option<NumericValue>,

}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Metric
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub name: Option<String>,
    pub expression: Option<String>,
    pub invisible: Option<bool>,
}
impl Metric {
    pub fn new(name: &str) -> Metric {
        Metric {
            name: Some(name.to_string()),
            ..Metric::default()
        }
    }
}
/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricAggregation
#[derive(Debug, Serialize, Deserialize)]
pub enum MetricAggregation {
    #[serde(rename = "METRIC_AGGREGATION_UNSPECIFIED")]
    MetricAggregationUnspecified,
    #[serde(rename = "TOTAL")]
    Total,
    #[serde(rename = "MINIMUM")]
    Minimum,
    #[serde(rename = "MAXIMUM")]
    Maximum,
    #[serde(rename = "COUNT")]
    Count,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricHeader
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MetricHeader {
    name: Option<String>,
    #[serde(rename = "type")]
    metric_type: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricHeader
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MetricMetadata {
    #[serde(rename = "apiName")]
    api_name: String,
    #[serde(rename = "uiName")]
    ui_name: String,
    description: String,
    #[serde(rename = "deprecatedApiNames")]
    deprecated_api_names: Vec<String>,
    #[serde(rename = "type")]
    metric_type: String,
    expression: String,
    #[serde(rename = "customDefinition")]
    custom_definition: bool,
    #[serde(rename = "blockedReasons")]
    blocked_reasons: Vec<String>,
    category: String,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricType
#[derive(Debug, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "METRIC_TYPE_UNSPECIFIED")]
    MetricTypeUnspecified,
    #[serde(rename = "TYPE_INTEGER")]
    TypeInteger,
    #[serde(rename = "TYPE_FLOAT")]
    TypeFloat,
    #[serde(rename = "TYPE_SECONDS")]
    TypeSeconds,
    #[serde(rename = "TYPE_MILLISECONDS")]
    TypeMilliseconds,
    #[serde(rename = "TYPE_MINUTES")]
    TypeMinutes,
    #[serde(rename = "TYPE_HOURS")]
    TypeHours,
    #[serde(rename = "TYPE_STANDARD")]
    TypeStandard,
    #[serde(rename = "TYPE_CURRENCY")]
    TypeCurrency,
    #[serde(rename = "TYPE_FEET")]
    TypeFeet,
    #[serde(rename = "TYPE_MILES")]
    TypeMiles,
    #[serde(rename = "TYPE_METERS")]
    TypeMeters,
    #[serde(rename = "TYPE_KILOMETERS")]
    TypeKilometers,

}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/OrderBy
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct OrderBy {}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MetricOrderBy {
    #[serde(rename = "metricName")]
    metric_name: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DimensionOrderBy {
    #[serde(rename = "dimensionName")]
    dimension_name: Option<String>,
    #[serde(rename = "orderType")]
    order_type: Option<OrderType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "ORDER_TYPE_UNSPECIFIED")]
    OrderTypeUnspecified,
    #[serde(rename = "ALPHANUMERIC")]
    Alphanumeric,
    #[serde(rename = "CASE_INSENSITIVE_ALPHANUMERIC")]
    CaseInsensitiveAlphanumeric,
    #[serde(rename = "NUMERIC")]
    Numeric,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PivotOrderBy {
    #[serde(rename = "metricName")]
    metric_name: Option<String>,
    #[serde(rename = "pivotSelections")]
    pivot_selections: Option<Vec<PivotSelection>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PivotSelection {
    #[serde(rename = "dimensionName")]
    dimension_name: Option<String>,
    #[serde(rename = "dimensionValue")]
    dimension_value: Option<String>,
}


/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Pivot
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Pivot {
    #[serde(rename = "fieldNames")]
    field_names: Option<Vec<String>>,
    #[serde(rename = "orderBys")]
    order_bys: Option<Vec<OrderBy>>,
    #[serde(rename = "offset")]
    offset: Option<String>,
    #[serde(rename = "limit")]
    limit: Option<String>,
    #[serde(rename = "metricAggregations")]
    metric_aggregations: Option<Vec<MetricAggregation>>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/PropertyQuota
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PropertyQuota {
    #[serde(rename = "tokensPerDay")]
    tokens_per_day: Option<QuotaStatus>,
    #[serde(rename = "tokensPerHour")]
    tokens_per_hour: Option<QuotaStatus>,
    #[serde(rename = "concurrentRequests")]
    concurrent_requests: Option<QuotaStatus>,
    #[serde(rename = "serverErrorsPerProjectPerHour")]
    server_errors_per_project_per_hour: Option<QuotaStatus>,
    #[serde(rename = "potentiallyThresholdedRequestsPerHour")]
    potentially_thresholded_requests_per_hour: Option<QuotaStatus>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct QuotaStatus {
    consumed: i32,
    remaining: i32,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/ResponseMetaData
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResponseMetaData {
    #[serde(rename = "dataLossFromOtherRow")]
    data_loss_from_other_row: Option<bool>,
    #[serde(rename = "schemaRestrictionResponse")]
    schema_restriction_response: Option<SchemaRestrictionResponse>,
    #[serde(rename = "currencyCode")]
    currency_code: Option<String>,
    #[serde(rename = "timeZone")]
    time_zone: Option<String>,
    #[serde(rename = "emptyReason")]
    empty_reason: Option<String>,
    #[serde(rename = "subjectToThresholding")]
    subject_to_thresholding: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SchemaRestrictionResponse {
    #[serde(rename = "activeMetricRestrictions")]
    active_metric_restrictions: Option<Vec<ActiveMetricRestriction>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActiveMetricRestriction {
    #[serde(rename = "restrictedMetricTypes")]
    restricted_metric_types: Option<Vec<RestrictedMetricType>>,
    #[serde(rename = "metricName")]
    metric_name: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestrictedMetricType {
    #[serde(rename = "RESTRICTED_METRIC_TYPE_UNSPECIFIED")]
    RestrictedMetricTypeUnspecified,
    #[serde(rename = "COST_DATA")]
    CostData,
    #[serde(rename = "REVENUE_DATA")]
    RevenueData,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Row
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "dimensionValues")]
    pub dimension_values: Option<Vec<DimensionValue>>,
    #[serde(rename = "metricValues")]
    pub metric_values: Option<Vec<MetricValue>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    pub value: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunPivotReportResponse
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunPivotReportResponse {
    #[serde(rename = "pivotHeaders")]
    pivot_headers: Option<Vec<PivotHeader>>,
    #[serde(rename = "dimensionHeaders")]
    dimension_headers: Option<Vec<DimensionHeader>>,
    #[serde(rename = "metricHeaders")]
    metric_headers: Option<Vec<MetricHeader>>,
    rows: Option<Vec<Row>>,
    aggregates: Option<Vec<Row>>,
    metadata: Option<ResponseMetaData>,
    #[serde(rename = "propertyQuota")]
    property_quota: Option<PropertyQuota>,
    kind: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PivotHeader {
    #[serde(rename = "pivotDimensionHeaders")]
    pivot_dimension_headers: Option<Vec<PivotDimensionHeader>>,
    #[serde(rename = "rowCount")]
    row_count: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PivotDimensionHeader {
    #[serde(rename = "dimensionValues")]
    dimension_values: Option<Vec<DimensionValue>>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunReportResponse
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RunReportResponse {
    #[serde(rename = "dimensionHeaders")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    #[serde(rename = "metricHeaders")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    pub rows: Option<Vec<Row>>,
    pub totals: Option<Vec<Row>>,
    pub maximums: Option<Vec<Row>>,
    pub minimums: Option<Vec<Row>>,
    #[serde(rename = "rowCount")]
    pub row_count: Option<i32>,
    pub metadata: Option<ResponseMetaData>,
    #[serde(rename = "propertyQuota")]
    pub property_quota: Option<PropertyQuota>,
    pub kind: Option<String>,
}
