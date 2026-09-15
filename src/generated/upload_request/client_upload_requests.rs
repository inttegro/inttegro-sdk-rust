//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro upload requests.
#[derive(Clone)]
pub struct UploadRequests {
    client: Client,
}

impl UploadRequests {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create an upload request
    pub async fn create(&self, request: &CreateUploadRequestRequest) -> Result<UploadRequest> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create an upload request with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateUploadRequestRequest,
        options: RequestOptions,
    ) -> Result<UploadRequest> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/upload_requests/create",
                    operation: "upload_requests.create",
                    field: Some("upload_request"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup an upload request
    pub async fn lookup(&self, request: &LookupUploadRequestRequest) -> Result<UploadRequest> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup an upload request with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupUploadRequestRequest,
        options: RequestOptions,
    ) -> Result<UploadRequest> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/upload_requests/lookup",
                    operation: "upload_requests.lookup",
                    field: Some("upload_request"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page upload requests
    pub async fn page(&self, request: &PageUploadRequestsRequest) -> Result<UploadRequestPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page upload requests with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageUploadRequestsRequest,
        options: RequestOptions,
    ) -> Result<UploadRequestPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/upload_requests/page",
                    operation: "upload_requests.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel an upload request
    pub async fn cancel(&self, request: &CancelUploadRequestRequest) -> Result<UploadRequest> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel an upload request with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelUploadRequestRequest,
        options: RequestOptions,
    ) -> Result<UploadRequest> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/upload_requests/cancel",
                    operation: "upload_requests.cancel",
                    field: Some("upload_request"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Review an upload request attempt
    pub async fn review(
        &self,
        request: &ReviewUploadRequestAttemptRequest,
    ) -> Result<UploadRequest> {
        self.review_with_options(request, RequestOptions::default())
            .await
    }
    /// Review an upload request attempt with per-request options.
    pub async fn review_with_options(
        &self,
        request: &ReviewUploadRequestAttemptRequest,
        options: RequestOptions,
    ) -> Result<UploadRequest> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/upload_requests/review",
                    operation: "upload_requests.review",
                    field: Some("upload_request"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Fulfill an upload request
    pub async fn fulfill(&self, request: FulfillUploadRequest) -> Result<UploadFulfillment> {
        self.fulfill_with_options(request, RequestOptions::default())
            .await
    }
    /// Fulfill an upload request with per-request options.
    pub async fn fulfill_with_options(
        &self,
        request: FulfillUploadRequest,
        options: RequestOptions,
    ) -> Result<UploadFulfillment> {
        self.client
            .fulfill_upload(
                "/upload_requests/upload",
                request,
                options,
                "upload_requests.fulfill",
            )
            .await
    }
}
