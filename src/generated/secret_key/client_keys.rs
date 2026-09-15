//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro keys.
#[derive(Clone)]
pub struct Keys {
    client: Client,
}

impl Keys {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Generate a secret key
    pub async fn generate(&self, request: &GenerateSecretKeyRequest) -> Result<GeneratedSecretKey> {
        self.generate_with_options(request, RequestOptions::default())
            .await
    }
    /// Generate a secret key with per-request options.
    pub async fn generate_with_options(
        &self,
        request: &GenerateSecretKeyRequest,
        options: RequestOptions,
    ) -> Result<GeneratedSecretKey> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/generate",
                    operation: "keys.generate",
                    field: Some("key"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page secret keys
    pub async fn page(&self, request: &PageSecretKeysRequest) -> Result<SecretKeyPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page secret keys with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageSecretKeysRequest,
        options: RequestOptions,
    ) -> Result<SecretKeyPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/page",
                    operation: "keys.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Retrieve a secret key
    pub async fn lookup(&self, request: &LookupSecretKeyRequest) -> Result<SecretKey> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Retrieve a secret key with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupSecretKeyRequest,
        options: RequestOptions,
    ) -> Result<SecretKey> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/lookup",
                    operation: "keys.lookup",
                    field: Some("key"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a secret key
    pub async fn update(&self, request: &UpdateSecretKeyRequest) -> Result<SecretKey> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a secret key with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateSecretKeyRequest,
        options: RequestOptions,
    ) -> Result<SecretKey> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/update",
                    operation: "keys.update",
                    field: Some("key"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Revoke a secret key
    pub async fn destroy(&self, request: &DestroySecretKeyRequest) -> Result<SecretKey> {
        self.destroy_with_options(request, RequestOptions::default())
            .await
    }
    /// Revoke a secret key with per-request options.
    pub async fn destroy_with_options(
        &self,
        request: &DestroySecretKeyRequest,
        options: RequestOptions,
    ) -> Result<SecretKey> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/destroy",
                    operation: "keys.destroy",
                    field: Some("key"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Retrieve secret key usage
    pub async fn usage(&self, request: &SecretKeyUsageRequest) -> Result<SecretKeyUsage> {
        self.usage_with_options(request, RequestOptions::default())
            .await
    }
    /// Retrieve secret key usage with per-request options.
    pub async fn usage_with_options(
        &self,
        request: &SecretKeyUsageRequest,
        options: RequestOptions,
    ) -> Result<SecretKeyUsage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/keys/usage",
                    operation: "keys.usage",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
