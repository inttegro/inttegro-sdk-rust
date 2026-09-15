//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro balance transactions.
#[derive(Clone)]
pub struct BalanceTransactions {
    client: Client,
}

impl BalanceTransactions {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Look up a balance transaction
    pub async fn lookup(
        &self,
        request: &LookupBalanceTransactionRequest,
    ) -> Result<BalanceTransaction> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a balance transaction with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupBalanceTransactionRequest,
        options: RequestOptions,
    ) -> Result<BalanceTransaction> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/balance_transactions/lookup",
                    operation: "balance_transactions.lookup",
                    field: Some("transaction"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through balance transactions
    pub async fn page(
        &self,
        request: &PageBalanceTransactionsRequest,
    ) -> Result<BalanceTransactionPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through balance transactions with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageBalanceTransactionsRequest,
        options: RequestOptions,
    ) -> Result<BalanceTransactionPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/balance_transactions/page",
                    operation: "balance_transactions.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
