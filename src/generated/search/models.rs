use crate::{Amount, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSearchOperator {
    Eq,
    In,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSearchSortField {
    Relevance,
    UpdatedAt,
    PublishedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSearchSortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResourceType {
    Customer,
    FinancialAccount,
    Order,
    Payout,
    Product,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSearchTotalRelation {
    Exact,
    LowerBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSearchFreshnessState {
    Current,
    Delayed,
    Partial,
    Unknown,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchFilter {
    pub field: String,
    pub operator: ResourceSearchOperator,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchFacet {
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchSort {
    pub field: ResourceSearchSortField,
    pub direction: ResourceSearchSortDirection,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ResourceSearchFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<ResourceSearchFacet>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<ResourceSearchSort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchTotal {
    pub value: i64,
    pub relation: ResourceSearchTotalRelation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchResourceTotal {
    pub resource_type: SearchResourceType,
    pub value: i64,
    pub relation: ResourceSearchTotalRelation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchResourceReference {
    pub r#type: SearchResourceType,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSearchResult {
    pub resource: ResourceSearchResourceReference,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchFacetBucket {
    pub value: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchFacetResult {
    pub field: String,
    pub buckets: Vec<ResourceSearchFacetBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchResourceFreshness {
    pub resource_type: SearchResourceType,
    pub state: ResourceSearchFreshnessState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_indexed_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSearchFreshness {
    pub state: ResourceSearchFreshnessState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceSearchResourceFreshness>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSearchPage {
    pub resource_types: Vec<SearchResourceType>,
    pub sort: ResourceSearchSort,
    pub page_size: u32,
    pub result_count: u32,
    pub has_more: bool,
    pub total: ResourceSearchTotal,
    pub resource_totals: Vec<ResourceSearchResourceTotal>,
    pub results: Vec<ResourceSearchResult>,
    pub facets: Vec<ResourceSearchFacetResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub freshness: ResourceSearchFreshness,
}
