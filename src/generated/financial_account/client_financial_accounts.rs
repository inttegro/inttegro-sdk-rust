//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

/// Operations for Inttegro financial accounts.
#[derive(Clone)]
pub struct FinancialAccounts {
    client: Client,
}

impl FinancialAccounts {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a financial account
    pub async fn create(
        &self,
        request: &FinancialAccountCreateRequest,
    ) -> Result<FinancialAccount> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a financial account with per-request options.
    pub async fn create_with_options(
        &self,
        request: &FinancialAccountCreateRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/create",
                    operation: "financial_accounts.create",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a financial account
    pub async fn lookup(&self, request: &FinancialAccountIDRequest) -> Result<FinancialAccount> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a financial account with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &FinancialAccountIDRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/lookup",
                    operation: "financial_accounts.lookup",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through financial accounts
    pub async fn page(
        &self,
        request: &FinancialAccountPageRequest,
    ) -> Result<FinancialAccountPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through financial accounts with per-request options.
    pub async fn page_with_options(
        &self,
        request: &FinancialAccountPageRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccountPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/page",
                    operation: "financial_accounts.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Connect a financial account
    pub async fn connect(
        &self,
        request: &FinancialAccountCreateRequest,
    ) -> Result<FinancialAccount> {
        self.connect_with_options(request, RequestOptions::default())
            .await
    }
    /// Connect a financial account with per-request options.
    pub async fn connect_with_options(
        &self,
        request: &FinancialAccountCreateRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/connect",
                    operation: "financial_accounts.connect",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a financial account
    pub async fn update(
        &self,
        request: &FinancialAccountUpdateRequest,
    ) -> Result<FinancialAccount> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a financial account with per-request options.
    pub async fn update_with_options(
        &self,
        request: &FinancialAccountUpdateRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/update",
                    operation: "financial_accounts.update",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Enable push capability
    pub async fn enable_push(
        &self,
        request: &FinancialAccountIDRequest,
    ) -> Result<FinancialAccount> {
        self.enable_push_with_options(request, RequestOptions::default())
            .await
    }
    /// Enable push capability with per-request options.
    pub async fn enable_push_with_options(
        &self,
        request: &FinancialAccountIDRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/enable_push",
                    operation: "financial_accounts.enable_push",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Disable push capability
    pub async fn disable_push(
        &self,
        request: &FinancialAccountDisableRequest,
    ) -> Result<FinancialAccount> {
        self.disable_push_with_options(request, RequestOptions::default())
            .await
    }
    /// Disable push capability with per-request options.
    pub async fn disable_push_with_options(
        &self,
        request: &FinancialAccountDisableRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/disable_push",
                    operation: "financial_accounts.disable_push",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Disconnect a financial account
    pub async fn disconnect(
        &self,
        request: &FinancialAccountDisableRequest,
    ) -> Result<FinancialAccount> {
        self.disconnect_with_options(request, RequestOptions::default())
            .await
    }
    /// Disconnect a financial account with per-request options.
    pub async fn disconnect_with_options(
        &self,
        request: &FinancialAccountDisableRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/disconnect",
                    operation: "financial_accounts.disconnect",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Reconnect a financial account
    pub async fn reconnect(&self, request: &FinancialAccountIDRequest) -> Result<FinancialAccount> {
        self.reconnect_with_options(request, RequestOptions::default())
            .await
    }
    /// Reconnect a financial account with per-request options.
    pub async fn reconnect_with_options(
        &self,
        request: &FinancialAccountIDRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/reconnect",
                    operation: "financial_accounts.reconnect",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Enable pull capability
    pub async fn enable_pull(
        &self,
        request: &FinancialAccountEnablePullRequest,
    ) -> Result<FinancialAccount> {
        self.enable_pull_with_options(request, RequestOptions::default())
            .await
    }
    /// Enable pull capability with per-request options.
    pub async fn enable_pull_with_options(
        &self,
        request: &FinancialAccountEnablePullRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/enable_pull",
                    operation: "financial_accounts.enable_pull",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Disable pull capability
    pub async fn disable_pull(
        &self,
        request: &FinancialAccountIDRequest,
    ) -> Result<FinancialAccount> {
        self.disable_pull_with_options(request, RequestOptions::default())
            .await
    }
    /// Disable pull capability with per-request options.
    pub async fn disable_pull_with_options(
        &self,
        request: &FinancialAccountIDRequest,
        options: RequestOptions,
    ) -> Result<FinancialAccount> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/financial_accounts/disable_pull",
                    operation: "financial_accounts.disable_pull",
                    field: Some("account"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}
