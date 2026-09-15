//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro specifications.
#[derive(Clone)]
pub struct Specifications {
    client: Client,
}

impl Specifications {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get country specifications
    pub async fn countries(&self) -> Result<CountrySpecifications> {
        self.countries_with_options(RequestOptions::default()).await
    }
    /// Get country specifications with per-request options.
    pub async fn countries_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<CountrySpecifications> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/spec/countries",
                    operation: "specifications.countries",
                    field: Some("countries"),
                    authenticated: false,
                },
                Some(&body),
                options,
            )
            .await
    }
}
