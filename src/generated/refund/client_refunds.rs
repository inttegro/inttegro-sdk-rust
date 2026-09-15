//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro refunds.
#[derive(Clone)]
pub struct Refunds {
    client: Client,
}

impl Refunds {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a refund
    pub async fn create(&self, request: &CreateRefundRequest) -> Result<Refund> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a refund with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/create",
                    operation: "refunds.create",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a refund
    pub async fn cancel(&self, request: &CancelRefundRequest) -> Result<Refund> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a refund with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/cancel",
                    operation: "refunds.cancel",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a refund
    pub async fn lookup(&self, request: &LookupRefundRequest) -> Result<Refund> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a refund with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/lookup",
                    operation: "refunds.lookup",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through refunds
    pub async fn page(&self, request: &PageRefundsRequest) -> Result<RefundPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through refunds with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageRefundsRequest,
        options: RequestOptions,
    ) -> Result<RefundPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/page",
                    operation: "refunds.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
