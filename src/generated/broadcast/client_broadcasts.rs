//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro broadcasts.
#[derive(Clone)]
pub struct Broadcasts {
    client: Client,
}

impl Broadcasts {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Look up a broadcast
    pub async fn lookup(&self, request: &LookupBroadcastRequest) -> Result<BroadcastDetail> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a broadcast with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupBroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/broadcasts/lookup",
                    operation: "broadcasts.lookup",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a broadcast
    pub async fn cancel(&self, request: &CancelBroadcastRequest) -> Result<BroadcastDetail> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a broadcast with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelBroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/broadcasts/cancel",
                    operation: "broadcasts.cancel",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
