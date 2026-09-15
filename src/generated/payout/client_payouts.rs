//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro payouts.
#[derive(Clone)]
pub struct Payouts {
    client: Client,
}

impl Payouts {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Schedule a payout
    pub async fn schedule(&self, request: &SchedulePayoutRequest) -> Result<Payout> {
        self.schedule_with_options(request, RequestOptions::default())
            .await
    }
    /// Schedule a payout with per-request options.
    pub async fn schedule_with_options(
        &self,
        request: &SchedulePayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/schedule",
                    operation: "payouts.schedule",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a payout
    pub async fn lookup(&self, request: &LookupPayoutRequest) -> Result<Payout> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a payout with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/lookup",
                    operation: "payouts.lookup",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Set payout destinations
    pub async fn set_destinations(
        &self,
        request: &SetPayoutDestinationsRequest,
    ) -> Result<PayoutSettingsMutation> {
        self.set_destinations_with_options(request, RequestOptions::default())
            .await
    }
    /// Set payout destinations with per-request options.
    pub async fn set_destinations_with_options(
        &self,
        request: &SetPayoutDestinationsRequest,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/set_destinations",
                    operation: "payouts.set_destinations",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Get payout settings
    pub async fn settings(&self) -> Result<PayoutSettingsLookup> {
        self.settings_with_options(RequestOptions::default()).await
    }
    /// Get payout settings with per-request options.
    pub async fn settings_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsLookup> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/settings",
                    operation: "payouts.settings",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Disable automatic payouts
    pub async fn disable(&self) -> Result<PayoutSettingsMutation> {
        self.disable_with_options(RequestOptions::default()).await
    }
    /// Disable automatic payouts with per-request options.
    pub async fn disable_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/disable",
                    operation: "payouts.disable",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Enable automatic payouts
    pub async fn enable(&self) -> Result<PayoutSettingsMutation> {
        self.enable_with_options(RequestOptions::default()).await
    }
    /// Enable automatic payouts with per-request options.
    pub async fn enable_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/enable",
                    operation: "payouts.enable",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Page through payouts
    pub async fn page(&self, request: &PagePayoutsRequest) -> Result<PayoutPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through payouts with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PagePayoutsRequest,
        options: RequestOptions,
    ) -> Result<PayoutPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/page",
                    operation: "payouts.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a scheduled payout
    pub async fn cancel(&self, request: &CancelPayoutRequest) -> Result<Payout> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a scheduled payout with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelPayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/cancel",
                    operation: "payouts.cancel",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
