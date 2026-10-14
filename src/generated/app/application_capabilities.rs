//! Typed application capability states.

use serde::{Deserialize, Serialize};

/// Current availability of an application capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationCapabilityStatus {
    Enabled,
    Restricted,
    PendingReview,
}

/// Public reason why an application capability is not enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationCapabilityStatusReason {
    SetupIncomplete,
    AccountUnderReview,
    AccountRestricted,
    AccountInactive,
}

/// A setup item that must be completed before a capability can be enabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCapabilityRequirement {
    pub code: String,
    pub description: String,
    pub due_at: Option<crate::Timestamp>,
}

/// Requirements that currently prevent a capability from being enabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCapabilityRequirements {
    pub currently_due: Vec<ApplicationCapabilityRequirement>,
}

/// The state of one commerce capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCapability {
    pub status: ApplicationCapabilityStatus,
    pub status_changed_at: crate::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<ApplicationCapabilityStatusReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<ApplicationCapabilityRequirements>,
}

/// Payment and payout capabilities available to the authenticated app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCapabilities {
    pub payments: ApplicationCapability,
    pub payouts: ApplicationCapability,
}
