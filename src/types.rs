use serde::{Deserialize, Serialize};

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/CohortSpec#CohortsRange
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CohortSpec {
    cohorts: Option<Vec<Cohort>>,
    #[serde(rename = "cohortsRange")]
    cohorts_range: Option<Vec<Cohort>>,
    #[serde(rename = "cohortReportSettings")]
    cohort_report_settings: Option<CohortReportSettings>,
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Cohort {
    name: Option<String>,
    dimension: Option<String>,
    #[serde(rename = "dateRange")]
    date_range: Option<DateRange>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CohortsRange {
    granularity: Option<String>,
    #[serde(rename = "startOffset")]
    start_offset: Option<i32>,
    #[serde(rename = "endOffset")]
    end_offset: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CohortReportSettings {
    accumulate: Option<bool>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DateRange
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
    pub fn from_string_vec(vals:Vec<&str>) -> Vec<Self>{
        let mut out = vec![];
        for val in vals{
            out.push(Self::new(val));
        }
        out
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionExpression {
    #[serde(rename = "lowerCase")]
    pub lower_case: Option<CaseExpression>,
    #[serde(rename = "upperCase")]
    pub upper_case: Option<CaseExpression>,
    #[serde(rename = "concatenate")]
    pub concatenate: Option<ConcatenateExpression>,

}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CaseExpression {
    #[serde(rename = "dimensionName")]
    pub dimension_name: Option<String>,

}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ConcatenateExpression {
    #[serde(rename = "dimensionNames")]
    pub dimension_names: Option<Vec<String>>,
    pub delimiter: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionHeader
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionHeader {
    pub name: Option<String>,
}


/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionMetadata
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionMetadata {
    #[serde(rename = "apiName")]
    pub api_name: Option<String>,
    #[serde(rename = "uiName")]
    pub ui_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "deprecatedApiNames")]
    pub deprecated_api_names: Option<Vec<String>>,
    #[serde(rename = "customDefinition")]
    pub custom_definition: Option<bool>,
    pub category: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/DimensionValue
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionValue {
    pub value: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/FilterExpression
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FilterExpression {
    #[serde(rename = "andGroup")]
    pub and_group: Option<FilterExpressionList>,
    #[serde(rename = "orGroup")]
    pub or_group: Option<FilterExpressionList>,
    #[serde(rename = "notExpression")]
    pub not_expression: Box<Option<FilterExpression>>,
    pub filter: Option<Filter>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FilterExpressionList {
    pub expressions: Option<Vec<FilterExpression>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct StringFilter {
    #[serde(rename = "matchType")]
    pub match_type: Option<MatchType>,
    pub value: Option<String>,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct InListFilter {
    values: Option<Vec<String>>,
    #[serde(rename = "caseSensitive")]
    case_sensitive: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct NumericFilter {
    operation: Option<Operation>,
    value: Option<NumericValue>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
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


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct NumericValue {
    #[serde(rename = "int64Value")]
    int_64_value: Option<String>,
    #[serde(rename = "doubleValue")]
    double_value: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BetweenFilter {
    #[serde(rename = "fromValue")]
    from_value: Option<NumericValue>,
    #[serde(rename = "toValue")]
    to_value: Option<NumericValue>,

}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Metric
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
    pub fn from_string_vec(vals:Vec<&str>) -> Vec<Self>{
        let mut out = vec![];
        for val in vals{
            out.push(Self::new(val));
        }
        out
    }
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricAggregation
#[derive(Debug, Serialize, Deserialize,Clone)]
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
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MetricHeader {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub metric_type: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricHeader
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MetricMetadata {
    #[serde(rename = "apiName")]
    pub api_name: String,
    #[serde(rename = "uiName")]
    pub ui_name: String,
    pub description: String,
    #[serde(rename = "deprecatedApiNames")]
    pub deprecated_api_names: Vec<String>,
    #[serde(rename = "type")]
    pub metric_type: String,
    pub expression: String,
    #[serde(rename = "customDefinition")]
    pub custom_definition: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub category: String,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/MetricType
#[derive(Debug, Serialize, Deserialize,Clone)]
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
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct OrderBy {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MetricOrderBy {
    #[serde(rename = "metricName")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DimensionOrderBy {
    #[serde(rename = "dimensionName")]
    pub dimension_name: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: Option<OrderType>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PivotOrderBy {
    #[serde(rename = "metricName")]
    pub metric_name: Option<String>,
    #[serde(rename = "pivotSelections")]
    pub pivot_selections: Option<Vec<PivotSelection>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PivotSelection {
    #[serde(rename = "dimensionName")]
    pub dimension_name: Option<String>,
    #[serde(rename = "dimensionValue")]
    pub dimension_value: Option<String>,
}


/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Pivot
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Pivot {
    #[serde(rename = "fieldNames")]
    pub field_names: Option<Vec<String>>,
    #[serde(rename = "orderBys")]
    pub order_bys: Option<Vec<OrderBy>>,
    #[serde(rename = "offset")]
    pub offset: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<String>,
    #[serde(rename = "metricAggregations")]
    pub metric_aggregations: Option<Vec<MetricAggregation>>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/PropertyQuota
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PropertyQuota {
    #[serde(rename = "tokensPerDay")]
    pub tokens_per_day: Option<QuotaStatus>,
    #[serde(rename = "tokensPerHour")]
    pub tokens_per_hour: Option<QuotaStatus>,
    #[serde(rename = "concurrentRequests")]
    pub concurrent_requests: Option<QuotaStatus>,
    #[serde(rename = "serverErrorsPerProjectPerHour")]
    pub server_errors_per_project_per_hour: Option<QuotaStatus>,
    #[serde(rename = "potentiallyThresholdedRequestsPerHour")]
    pub potentially_thresholded_requests_per_hour: Option<QuotaStatus>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct QuotaStatus {
    pub consumed: i32,
    pub remaining: i32,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/ResponseMetaData
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMetaData {
    #[serde(rename = "dataLossFromOtherRow")]
    pub data_loss_from_other_row: Option<bool>,
    #[serde(rename = "schemaRestrictionResponse")]
    pub schema_restriction_response: Option<SchemaRestrictionResponse>,
    #[serde(rename = "currencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<String>,
    #[serde(rename = "subjectToThresholding")]
    pub subject_to_thresholding: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SchemaRestrictionResponse {
    #[serde(rename = "activeMetricRestrictions")]
    pub active_metric_restrictions: Option<Vec<ActiveMetricRestriction>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ActiveMetricRestriction {
    #[serde(rename = "restrictedMetricTypes")]
    pub restricted_metric_types: Option<Vec<RestrictedMetricType>>,
    #[serde(rename = "metricName")]
    pub metric_name: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum RestrictedMetricType {
    #[serde(rename = "RESTRICTED_METRIC_TYPE_UNSPECIFIED")]
    RestrictedMetricTypeUnspecified,
    #[serde(rename = "COST_DATA")]
    CostData,
    #[serde(rename = "REVENUE_DATA")]
    RevenueData,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/Row
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    #[serde(rename = "dimensionValues")]
    pub dimension_values: Option<Vec<DimensionValue>>,
    #[serde(rename = "metricValues")]
    pub metric_values: Option<Vec<MetricValue>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MetricValue {
    pub value: Option<String>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunPivotReportResponse
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct RunPivotReportResponse {
    #[serde(rename = "pivotHeaders")]
    pub pivot_headers: Option<Vec<PivotHeader>>,
    #[serde(rename = "dimensionHeaders")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    #[serde(rename = "metricHeaders")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    pub rows: Option<Vec<Row>>,
    pub aggregates: Option<Vec<Row>>,
    pub metadata: Option<ResponseMetaData>,
    #[serde(rename = "propertyQuota")]
    pub property_quota: Option<PropertyQuota>,
    pub kind: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PivotHeader {
    #[serde(rename = "pivotDimensionHeaders")]
    pub pivot_dimension_headers: Option<Vec<PivotDimensionHeader>>,
    #[serde(rename = "rowCount")]
    pub row_count: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PivotDimensionHeader {
    #[serde(rename = "dimensionValues")]
    pub dimension_values: Option<Vec<DimensionValue>>,
}

/// https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/RunReportResponse
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
