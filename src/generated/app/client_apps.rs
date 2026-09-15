//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro apps.
#[derive(Clone)]
pub struct Apps {
    client: Client,
}

impl Apps {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create an application
    pub async fn create(&self, request: &CreateApplicationRequest) -> Result<Application> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create an application with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateApplicationRequest,
        options: RequestOptions,
    ) -> Result<Application> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/apps/create",
                    operation: "apps.create",
                    field: Some("app"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Retrieve the authenticated application
    pub async fn lookup(&self) -> Result<Application> {
        self.lookup_with_options(RequestOptions::default()).await
    }
    /// Retrieve the authenticated application with per-request options.
    pub async fn lookup_with_options(&self, options: RequestOptions) -> Result<Application> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/apps/lookup",
                    operation: "apps.lookup",
                    field: Some("app"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Update the authenticated application
    pub async fn update(&self, request: &UpdateApplicationRequest) -> Result<Application> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update the authenticated application with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateApplicationRequest,
        options: RequestOptions,
    ) -> Result<Application> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/apps/update",
                    operation: "apps.update",
                    field: Some("app"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
