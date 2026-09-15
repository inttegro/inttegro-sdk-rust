//! Generated, typed Inttegro domain or request value. Do not edit manually.

use serde::{Deserialize, Serialize};

/// Typed Inttegro domain value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductMedia {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_page_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infographic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demo_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gallery: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<Vec<String>>,
}
