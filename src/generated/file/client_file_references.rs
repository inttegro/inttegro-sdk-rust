//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro file references.
#[derive(Clone)]
pub struct FileReferences {
    client: Client,
}

impl FileReferences {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Reconcile file references
    pub async fn reconcile(
        &self,
        request: &FileReferenceReconcileRequest,
    ) -> Result<FileReferenceReconciliation> {
        self.reconcile_with_options(request, RequestOptions::default())
            .await
    }
    /// Reconcile file references with per-request options.
    pub async fn reconcile_with_options(
        &self,
        request: &FileReferenceReconcileRequest,
        options: RequestOptions,
    ) -> Result<FileReferenceReconciliation> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/file_references/reconcile",
                    operation: "file_references.reconcile",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
