//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro files.
#[derive(Clone)]
pub struct Files {
    client: Client,
}

impl Files {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a file
    pub async fn create(&self, request: CreateFileRequest) -> Result<File> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a file with per-request options.
    pub async fn create_with_options(
        &self,
        request: CreateFileRequest,
        options: RequestOptions,
    ) -> Result<File> {
        self.client
            .upload_file(
                "/files/create",
                request,
                options,
                "files.create",
                Some("file"),
                true,
            )
            .await
    }

    /// Lookup a file
    pub async fn lookup(&self, request: &LookupFileRequest) -> Result<File> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a file with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupFileRequest,
        options: RequestOptions,
    ) -> Result<File> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/files/lookup",
                    operation: "files.lookup",
                    field: Some("file"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page files
    pub async fn page(&self, request: &PageFilesRequest) -> Result<FilePage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page files with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageFilesRequest,
        options: RequestOptions,
    ) -> Result<FilePage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/files/page",
                    operation: "files.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Deliver file contents
    pub async fn contents(&self, request: &FileContentsRequest) -> Result<FileDownload> {
        self.contents_with_options(request, RequestOptions::default())
            .await
    }
    /// Deliver file contents with per-request options.
    pub async fn contents_with_options(
        &self,
        request: &FileContentsRequest,
        options: RequestOptions,
    ) -> Result<FileDownload> {
        self.client
            .download(
                "POST",
                "/files/contents",
                Some(request),
                options,
                "files.contents",
                true,
            )
            .await
    }

    /// Delete a file
    pub async fn delete(&self, request: &DeleteFileRequest) -> Result<File> {
        self.delete_with_options(request, RequestOptions::default())
            .await
    }
    /// Delete a file with per-request options.
    pub async fn delete_with_options(
        &self,
        request: &DeleteFileRequest,
        options: RequestOptions,
    ) -> Result<File> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/files/delete",
                    operation: "files.delete",
                    field: Some("file"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
