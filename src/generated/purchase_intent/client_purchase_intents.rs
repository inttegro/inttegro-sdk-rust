//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro purchase intents.
#[derive(Clone)]
pub struct PurchaseIntents {
    client: Client,
}

impl PurchaseIntents {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a purchase intent
    pub async fn create(&self, request: &CreatePurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a purchase intent with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreatePurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/create",
                    operation: "purchase_intents.create",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a purchase intent
    pub async fn update(&self, request: &UpdatePurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a purchase intent with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdatePurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/update",
                    operation: "purchase_intents.update",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a purchase intent
    pub async fn cancel(&self, request: &CancelPurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a purchase intent with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelPurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/cancel",
                    operation: "purchase_intents.cancel",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a purchase intent
    pub async fn lookup(&self, request: &LookupPurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a purchase intent with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/lookup",
                    operation: "purchase_intents.lookup",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// List purchase intents
    pub async fn page(&self, request: &PagePurchaseIntentsRequest) -> Result<PurchaseIntentPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// List purchase intents with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PagePurchaseIntentsRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntentPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/page",
                    operation: "purchase_intents.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
