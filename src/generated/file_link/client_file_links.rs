//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro file links.
#[derive(Clone)]
pub struct FileLinks {
    client: Client,
}

impl FileLinks {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a file link
    pub async fn create(&self, request: &CreateFileLinkRequest) -> Result<FileLinkCreation> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a file link with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateFileLinkRequest,
        options: RequestOptions,
    ) -> Result<FileLinkCreation> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/file_links/create",
                    operation: "file_links.create",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a file link
    pub async fn lookup(&self, request: &LookupFileLinkRequest) -> Result<FileLink> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a file link with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupFileLinkRequest,
        options: RequestOptions,
    ) -> Result<FileLink> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/file_links/lookup",
                    operation: "file_links.lookup",
                    field: Some("file_link"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page file links
    pub async fn page(&self, request: &PageFileLinksRequest) -> Result<FileLinkPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page file links with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageFileLinksRequest,
        options: RequestOptions,
    ) -> Result<FileLinkPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/file_links/page",
                    operation: "file_links.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Revoke a file link
    pub async fn revoke(&self, request: &RevokeFileLinkRequest) -> Result<FileLink> {
        self.revoke_with_options(request, RequestOptions::default())
            .await
    }
    /// Revoke a file link with per-request options.
    pub async fn revoke_with_options(
        &self,
        request: &RevokeFileLinkRequest,
        options: RequestOptions,
    ) -> Result<FileLink> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/file_links/revoke",
                    operation: "file_links.revoke",
                    field: Some("file_link"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Open a public file link
    pub async fn open(&self, request: OpenFileLinkRequest) -> Result<FileDownload> {
        self.client
            .open_file_link("/file_links/open", request, "file_links.open")
            .await
    }
}
