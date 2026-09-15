//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro balances.
#[derive(Clone)]
pub struct Balances {
    client: Client,
}

impl Balances {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve your balance
    pub async fn get(&self) -> Result<BalanceSnapshot> {
        self.get_with_options(RequestOptions::default()).await
    }
    /// Retrieve your balance with per-request options.
    pub async fn get_with_options(&self, options: RequestOptions) -> Result<BalanceSnapshot> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/balances",
                    operation: "balances.get",
                    field: Some("balances"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }
}
