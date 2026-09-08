//! Typed API resources.

use crate::generated::*;
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

/// Operations for Inttegro broadcasts.
#[derive(Clone)]
pub struct Broadcasts {
    client: Client,
}

impl Broadcasts {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Look up a broadcast
    pub async fn lookup(&self, request: &LookupBroadcastRequest) -> Result<BroadcastDetail> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a broadcast with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupBroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/broadcasts/lookup",
                    operation: "broadcasts.lookup",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a broadcast
    pub async fn cancel(&self, request: &CancelBroadcastRequest) -> Result<BroadcastDetail> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a broadcast with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelBroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/broadcasts/cancel",
                    operation: "broadcasts.cancel",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro chimes.
#[derive(Clone)]
pub struct Chimes {
    client: Client,
}

impl Chimes {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a Chime
    pub async fn send(&self, request: &SendChimeRequest) -> Result<Chime> {
        self.send_with_options(request, RequestOptions::default())
            .await
    }
    /// Send a Chime with per-request options.
    pub async fn send_with_options(
        &self,
        request: &SendChimeRequest,
        options: RequestOptions,
    ) -> Result<Chime> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/send",
                    operation: "chimes.send",
                    field: Some("chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a Chime
    pub async fn lookup(&self, request: &LookupChimeRequest) -> Result<Chime> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a Chime with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupChimeRequest,
        options: RequestOptions,
    ) -> Result<Chime> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/lookup",
                    operation: "chimes.lookup",
                    field: Some("chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through Chimes
    pub async fn page(&self, request: &PageChimesRequest) -> Result<ChimePage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through Chimes with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageChimesRequest,
        options: RequestOptions,
    ) -> Result<ChimePage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/page",
                    operation: "chimes.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Schedule Chimes
    pub async fn schedule(&self, request: &ScheduleChimeRequest) -> Result<ScheduleCreationDetail> {
        self.schedule_with_options(request, RequestOptions::default())
            .await
    }
    /// Schedule Chimes with per-request options.
    pub async fn schedule_with_options(
        &self,
        request: &ScheduleChimeRequest,
        options: RequestOptions,
    ) -> Result<ScheduleCreationDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/schedule",
                    operation: "chimes.schedule",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Broadcast Chimes
    pub async fn broadcast(&self, request: &BroadcastRequest) -> Result<BroadcastCreationDetail> {
        self.broadcast_with_options(request, RequestOptions::default())
            .await
    }
    /// Broadcast Chimes with per-request options.
    pub async fn broadcast_with_options(
        &self,
        request: &BroadcastRequest,
        options: RequestOptions,
    ) -> Result<BroadcastCreationDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/chimes/broadcast",
                    operation: "chimes.broadcast",
                    field: Some("broadcast"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro customers.
#[derive(Clone)]
pub struct Customers {
    client: Client,
}

impl Customers {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a customer
    pub async fn create(&self, request: &CreateCustomerRequest) -> Result<Customer> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a customer with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateCustomerRequest,
        options: RequestOptions,
    ) -> Result<Customer> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/customers/create",
                    operation: "customers.create",
                    field: Some("customer"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a customer
    pub async fn lookup(&self, request: &LookupCustomerRequest) -> Result<Customer> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a customer with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupCustomerRequest,
        options: RequestOptions,
    ) -> Result<Customer> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/customers/lookup",
                    operation: "customers.lookup",
                    field: Some("customer"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a customer
    pub async fn update(&self, request: &UpdateCustomerRequest) -> Result<Customer> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a customer with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateCustomerRequest,
        options: RequestOptions,
    ) -> Result<Customer> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/customers/update",
                    operation: "customers.update",
                    field: Some("customer"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through customers
    pub async fn page(&self, request: &PageCustomersRequest) -> Result<CustomerPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through customers with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageCustomersRequest,
        options: RequestOptions,
    ) -> Result<CustomerPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/customers/page",
                    operation: "customers.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

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

/// Operations for Inttegro message templates.
#[derive(Clone)]
pub struct MessageTemplates {
    client: Client,
}

impl MessageTemplates {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a message template
    pub async fn create(&self, request: &CreateMessageTemplateRequest) -> Result<MessageTemplate> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a message template with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateMessageTemplateRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/create",
                    operation: "message_templates.create",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a message template
    pub async fn update(&self, request: &UpdateMessageTemplateRequest) -> Result<MessageTemplate> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a message template with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateMessageTemplateRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/update",
                    operation: "message_templates.update",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Publish a message template
    pub async fn publish(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.publish_with_options(request, RequestOptions::default())
            .await
    }
    /// Publish a message template with per-request options.
    pub async fn publish_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/publish",
                    operation: "message_templates.publish",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Archive a message template
    pub async fn archive(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.archive_with_options(request, RequestOptions::default())
            .await
    }
    /// Archive a message template with per-request options.
    pub async fn archive_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/archive",
                    operation: "message_templates.archive",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a message template
    pub async fn lookup(&self, request: &MessageTemplateIDRequest) -> Result<MessageTemplate> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a message template with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &MessageTemplateIDRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplate> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/lookup",
                    operation: "message_templates.lookup",
                    field: Some("message_template"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page message templates
    pub async fn page(
        &self,
        request: &PageMessageTemplatesRequest,
    ) -> Result<MessageTemplatesPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page message templates with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageMessageTemplatesRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplatesPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/page",
                    operation: "message_templates.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Render a message template preview
    pub async fn render_preview(
        &self,
        request: &RenderMessageTemplatePreviewRequest,
    ) -> Result<MessageTemplatePreview> {
        self.render_preview_with_options(request, RequestOptions::default())
            .await
    }
    /// Render a message template preview with per-request options.
    pub async fn render_preview_with_options(
        &self,
        request: &RenderMessageTemplatePreviewRequest,
        options: RequestOptions,
    ) -> Result<MessageTemplatePreview> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/message_templates/render_preview",
                    operation: "message_templates.render_preview",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro orders.
#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new order
    pub async fn create(&self, request: &CreateOrderRequest) -> Result<Order> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a new order with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/create",
                    operation: "orders.create",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup an order
    pub async fn lookup(&self, request: &LookupOrderRequest) -> Result<Order> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup an order with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/lookup",
                    operation: "orders.lookup",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update an order
    pub async fn update(&self, request: &UpdateOrderRequest) -> Result<Order> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update an order with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/update",
                    operation: "orders.update",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Pay for an order
    pub async fn pay(&self, request: &PayOrderRequest) -> Result<Order> {
        self.pay_with_options(request, RequestOptions::default())
            .await
    }
    /// Pay for an order with per-request options.
    pub async fn pay_with_options(
        &self,
        request: &PayOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/pay",
                    operation: "orders.pay",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Confirm payment with token
    pub async fn confirm_payment(&self, request: &ConfirmPaymentRequest) -> Result<Order> {
        self.confirm_payment_with_options(request, RequestOptions::default())
            .await
    }
    /// Confirm payment with token with per-request options.
    pub async fn confirm_payment_with_options(
        &self,
        request: &ConfirmPaymentRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/confirm_payment",
                    operation: "orders.confirm_payment",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Request payment confirmation
    pub async fn request_confirmation(
        &self,
        request: &RequestConfirmationRequest,
    ) -> Result<Order> {
        self.request_confirmation_with_options(request, RequestOptions::default())
            .await
    }
    /// Request payment confirmation with per-request options.
    pub async fn request_confirmation_with_options(
        &self,
        request: &RequestConfirmationRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/request_confirmation",
                    operation: "orders.request_confirmation",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel an order
    pub async fn cancel(&self, request: &CancelOrderRequest) -> Result<Order> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel an order with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/cancel",
                    operation: "orders.cancel",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Finalize an order
    pub async fn finalize(&self, request: &FinalizeOrderRequest) -> Result<Order> {
        self.finalize_with_options(request, RequestOptions::default())
            .await
    }
    /// Finalize an order with per-request options.
    pub async fn finalize_with_options(
        &self,
        request: &FinalizeOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/finalize",
                    operation: "orders.finalize",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Complete an order
    pub async fn complete(&self, request: &CompleteOrderRequest) -> Result<Order> {
        self.complete_with_options(request, RequestOptions::default())
            .await
    }
    /// Complete an order with per-request options.
    pub async fn complete_with_options(
        &self,
        request: &CompleteOrderRequest,
        options: RequestOptions,
    ) -> Result<Order> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/complete",
                    operation: "orders.complete",
                    field: Some("order"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Send an order invoice
    pub async fn send_invoice(
        &self,
        request: &OrderDocumentDeliveryRequest,
    ) -> Result<OrderDocumentDeliveryResult> {
        self.send_invoice_with_options(request, RequestOptions::default())
            .await
    }
    /// Send an order invoice with per-request options.
    pub async fn send_invoice_with_options(
        &self,
        request: &OrderDocumentDeliveryRequest,
        options: RequestOptions,
    ) -> Result<OrderDocumentDeliveryResult> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/send_invoice",
                    operation: "orders.send_invoice",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Send an order receipt
    pub async fn send_receipt(
        &self,
        request: &OrderDocumentDeliveryRequest,
    ) -> Result<OrderDocumentDeliveryResult> {
        self.send_receipt_with_options(request, RequestOptions::default())
            .await
    }
    /// Send an order receipt with per-request options.
    pub async fn send_receipt_with_options(
        &self,
        request: &OrderDocumentDeliveryRequest,
        options: RequestOptions,
    ) -> Result<OrderDocumentDeliveryResult> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/send_receipt",
                    operation: "orders.send_receipt",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through orders
    pub async fn page(&self, request: &PageOrdersRequest) -> Result<OrderPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through orders with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageOrdersRequest,
        options: RequestOptions,
    ) -> Result<OrderPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/page",
                    operation: "orders.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Create a refund using the compatibility URL
    pub async fn refund(&self, request: &CreateRefundRequest) -> Result<Refund> {
        self.refund_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a refund using the compatibility URL with per-request options.
    pub async fn refund_with_options(
        &self,
        request: &CreateRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/orders/refund",
                    operation: "orders.refund",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro otp.
#[derive(Clone)]
pub struct Otp {
    client: Client,
}

impl Otp {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Initiate OTP transaction
    pub async fn initiate(&self, request: &InitiateOTPRequest) -> Result<OTPTransaction> {
        self.initiate_with_options(request, RequestOptions::default())
            .await
    }
    /// Initiate OTP transaction with per-request options.
    pub async fn initiate_with_options(
        &self,
        request: &InitiateOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPTransaction> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/initiate",
                    operation: "otp.initiate",
                    field: Some("transaction"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Verify OTP token
    pub async fn verify(&self, request: &VerifyOTPRequest) -> Result<OTPVerification> {
        self.verify_with_options(request, RequestOptions::default())
            .await
    }
    /// Verify OTP token with per-request options.
    pub async fn verify_with_options(
        &self,
        request: &VerifyOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPVerification> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/verify",
                    operation: "otp.verify",
                    field: None,
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup OTP transaction
    pub async fn lookup(&self, request: &LookupOTPRequest) -> Result<OTPTransaction> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup OTP transaction with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupOTPRequest,
        options: RequestOptions,
    ) -> Result<OTPTransaction> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/otp/lookup",
                    operation: "otp.lookup",
                    field: Some("transaction"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro payment methods.
#[derive(Clone)]
pub struct PaymentMethods {
    client: Client,
}

impl PaymentMethods {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Tokenize a payment method
    pub async fn tokenize(
        &self,
        request: &TokenizeMobileMoneyPaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        self.tokenize_with_options(request, RequestOptions::default())
            .await
    }
    /// Tokenize a payment method with per-request options.
    pub async fn tokenize_with_options(
        &self,
        request: &TokenizeMobileMoneyPaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/tokenize",
                    operation: "payment_methods.tokenize",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a payment method
    pub async fn lookup(&self, request: &LookupPaymentMethodRequest) -> Result<PaymentMethod> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a payment method with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/lookup",
                    operation: "payment_methods.lookup",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page payment methods
    pub async fn page(&self, request: &PaymentMethodPageRequest) -> Result<PaymentMethodPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page payment methods with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PaymentMethodPageRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethodPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/page",
                    operation: "payment_methods.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a payment method
    pub async fn update(&self, request: &UpdatePaymentMethodRequest) -> Result<PaymentMethod> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a payment method with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdatePaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/update",
                    operation: "payment_methods.update",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Activate a payment method
    pub async fn activate(&self, request: &ActivatePaymentMethodRequest) -> Result<PaymentMethod> {
        self.activate_with_options(request, RequestOptions::default())
            .await
    }
    /// Activate a payment method with per-request options.
    pub async fn activate_with_options(
        &self,
        request: &ActivatePaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/activate",
                    operation: "payment_methods.activate",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Deactivate a payment method
    pub async fn deactivate(
        &self,
        request: &DisactivatePaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        self.deactivate_with_options(request, RequestOptions::default())
            .await
    }
    /// Deactivate a payment method with per-request options.
    pub async fn deactivate_with_options(
        &self,
        request: &DisactivatePaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/disactivate",
                    operation: "payment_methods.deactivate",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Archive a payment method
    pub async fn archive(&self, request: &ArchivePaymentMethodRequest) -> Result<PaymentMethod> {
        self.archive_with_options(request, RequestOptions::default())
            .await
    }
    /// Archive a payment method with per-request options.
    pub async fn archive_with_options(
        &self,
        request: &ArchivePaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/archive",
                    operation: "payment_methods.archive",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Unarchive a payment method
    pub async fn unarchive(
        &self,
        request: &UnarchivePaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        self.unarchive_with_options(request, RequestOptions::default())
            .await
    }
    /// Unarchive a payment method with per-request options.
    pub async fn unarchive_with_options(
        &self,
        request: &UnarchivePaymentMethodRequest,
        options: RequestOptions,
    ) -> Result<PaymentMethod> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/unarchive",
                    operation: "payment_methods.unarchive",
                    field: Some("payment_method"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Get payment method settings
    pub async fn settings(&self) -> Result<PaymentMethodSettings> {
        self.settings_with_options(RequestOptions::default()).await
    }
    /// Get payment method settings with per-request options.
    pub async fn settings_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PaymentMethodSettings> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payment_methods/settings",
                    operation: "payment_methods.settings",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }
}

/// Operations for Inttegro payouts.
#[derive(Clone)]
pub struct Payouts {
    client: Client,
}

impl Payouts {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Schedule a payout
    pub async fn schedule(&self, request: &SchedulePayoutRequest) -> Result<Payout> {
        self.schedule_with_options(request, RequestOptions::default())
            .await
    }
    /// Schedule a payout with per-request options.
    pub async fn schedule_with_options(
        &self,
        request: &SchedulePayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/schedule",
                    operation: "payouts.schedule",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a payout
    pub async fn lookup(&self, request: &LookupPayoutRequest) -> Result<Payout> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a payout with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/lookup",
                    operation: "payouts.lookup",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Set payout destinations
    pub async fn set_destinations(
        &self,
        request: &SetPayoutDestinationsRequest,
    ) -> Result<PayoutSettingsMutation> {
        self.set_destinations_with_options(request, RequestOptions::default())
            .await
    }
    /// Set payout destinations with per-request options.
    pub async fn set_destinations_with_options(
        &self,
        request: &SetPayoutDestinationsRequest,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/set_destinations",
                    operation: "payouts.set_destinations",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Get payout settings
    pub async fn settings(&self) -> Result<PayoutSettingsLookup> {
        self.settings_with_options(RequestOptions::default()).await
    }
    /// Get payout settings with per-request options.
    pub async fn settings_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsLookup> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/settings",
                    operation: "payouts.settings",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Disable automatic payouts
    pub async fn disable(&self) -> Result<PayoutSettingsMutation> {
        self.disable_with_options(RequestOptions::default()).await
    }
    /// Disable automatic payouts with per-request options.
    pub async fn disable_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/disable",
                    operation: "payouts.disable",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Enable automatic payouts
    pub async fn enable(&self) -> Result<PayoutSettingsMutation> {
        self.enable_with_options(RequestOptions::default()).await
    }
    /// Enable automatic payouts with per-request options.
    pub async fn enable_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<PayoutSettingsMutation> {
        let body = serde_json::json!({});
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/enable",
                    operation: "payouts.enable",
                    field: Some("settings"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// Page through payouts
    pub async fn page(&self, request: &PagePayoutsRequest) -> Result<PayoutPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through payouts with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PagePayoutsRequest,
        options: RequestOptions,
    ) -> Result<PayoutPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/page",
                    operation: "payouts.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a scheduled payout
    pub async fn cancel(&self, request: &CancelPayoutRequest) -> Result<Payout> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a scheduled payout with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelPayoutRequest,
        options: RequestOptions,
    ) -> Result<Payout> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/payouts/cancel",
                    operation: "payouts.cancel",
                    field: Some("payout"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro prices.
#[derive(Clone)]
pub struct Prices {
    client: Client,
}

impl Prices {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a price
    pub async fn create(&self, request: &CatalogPriceParams) -> Result<CatalogPrice> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a price with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CatalogPriceParams,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/create",
                    operation: "prices.create",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a price
    pub async fn lookup(&self, request: &LookupPriceRequest) -> Result<CatalogPrice> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a price with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPriceRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/lookup",
                    operation: "prices.lookup",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through prices
    pub async fn page(&self, request: &PricePageRequest) -> Result<PricePage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through prices with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PricePageRequest,
        options: RequestOptions,
    ) -> Result<PricePage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/page",
                    operation: "prices.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a price
    pub async fn update(&self, request: &UpdatePriceRequest) -> Result<CatalogPrice> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a price with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdatePriceRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/update",
                    operation: "prices.update",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Activate a price
    pub async fn activate(&self, request: &PriceActionRequest) -> Result<CatalogPrice> {
        self.activate_with_options(request, RequestOptions::default())
            .await
    }
    /// Activate a price with per-request options.
    pub async fn activate_with_options(
        &self,
        request: &PriceActionRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/activate",
                    operation: "prices.activate",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Deactivate a price
    pub async fn deactivate(&self, request: &PriceActionRequest) -> Result<CatalogPrice> {
        self.deactivate_with_options(request, RequestOptions::default())
            .await
    }
    /// Deactivate a price with per-request options.
    pub async fn deactivate_with_options(
        &self,
        request: &PriceActionRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/deactivate",
                    operation: "prices.deactivate",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Archive a price
    pub async fn archive(&self, request: &PriceActionRequest) -> Result<CatalogPrice> {
        self.archive_with_options(request, RequestOptions::default())
            .await
    }
    /// Archive a price with per-request options.
    pub async fn archive_with_options(
        &self,
        request: &PriceActionRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/prices/archive",
                    operation: "prices.archive",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro products.
#[derive(Clone)]
pub struct Products {
    client: Client,
}

impl Products {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a product
    pub async fn create(&self, request: &CreateProductRequest) -> Result<Product> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a product with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateProductRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/create",
                    operation: "products.create",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Add a price to a product
    pub async fn add_price(&self, request: &AddProductPriceRequest) -> Result<CatalogPrice> {
        self.add_price_with_options(request, RequestOptions::default())
            .await
    }
    /// Add a price to a product with per-request options.
    pub async fn add_price_with_options(
        &self,
        request: &AddProductPriceRequest,
        options: RequestOptions,
    ) -> Result<CatalogPrice> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/add_price",
                    operation: "products.add_price",
                    field: Some("price"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a product
    pub async fn lookup(&self, request: &LookupProductRequest) -> Result<Product> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a product with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupProductRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/lookup",
                    operation: "products.lookup",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a product
    pub async fn update(&self, request: &UpdateProductRequest) -> Result<Product> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a product with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdateProductRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/update",
                    operation: "products.update",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Publish a product
    pub async fn publish(&self, request: &ProductActionRequest) -> Result<Product> {
        self.publish_with_options(request, RequestOptions::default())
            .await
    }
    /// Publish a product with per-request options.
    pub async fn publish_with_options(
        &self,
        request: &ProductActionRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/publish",
                    operation: "products.publish",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Unpublish a product
    pub async fn unpublish(&self, request: &ProductActionRequest) -> Result<Product> {
        self.unpublish_with_options(request, RequestOptions::default())
            .await
    }
    /// Unpublish a product with per-request options.
    pub async fn unpublish_with_options(
        &self,
        request: &ProductActionRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/unpublish",
                    operation: "products.unpublish",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Archive a product
    pub async fn archive(&self, request: &ProductActionRequest) -> Result<Product> {
        self.archive_with_options(request, RequestOptions::default())
            .await
    }
    /// Archive a product with per-request options.
    pub async fn archive_with_options(
        &self,
        request: &ProductActionRequest,
        options: RequestOptions,
    ) -> Result<Product> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/archive",
                    operation: "products.archive",
                    field: Some("product"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through products
    pub async fn page(&self, request: &PageProductsRequest) -> Result<ProductPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through products with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageProductsRequest,
        options: RequestOptions,
    ) -> Result<ProductPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/products/page",
                    operation: "products.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro purchase intents.
#[derive(Clone)]
pub struct PurchaseIntents {
    client: Client,
}

impl PurchaseIntents {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a purchase intent
    pub async fn create(&self, request: &CreatePurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a purchase intent with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreatePurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/create",
                    operation: "purchase_intents.create",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Update a purchase intent
    pub async fn update(&self, request: &UpdatePurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }
    /// Update a purchase intent with per-request options.
    pub async fn update_with_options(
        &self,
        request: &UpdatePurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/update",
                    operation: "purchase_intents.update",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a purchase intent
    pub async fn cancel(&self, request: &CancelPurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a purchase intent with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelPurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/cancel",
                    operation: "purchase_intents.cancel",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Lookup a purchase intent
    pub async fn lookup(&self, request: &LookupPurchaseIntentRequest) -> Result<PurchaseIntent> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Lookup a purchase intent with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupPurchaseIntentRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntent> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/lookup",
                    operation: "purchase_intents.lookup",
                    field: Some("purchase_intent"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// List purchase intents
    pub async fn page(&self, request: &PagePurchaseIntentsRequest) -> Result<PurchaseIntentPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// List purchase intents with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PagePurchaseIntentsRequest,
        options: RequestOptions,
    ) -> Result<PurchaseIntentPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/purchase_intents/page",
                    operation: "purchase_intents.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro refunds.
#[derive(Clone)]
pub struct Refunds {
    client: Client,
}

impl Refunds {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a refund
    pub async fn create(&self, request: &CreateRefundRequest) -> Result<Refund> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }
    /// Create a refund with per-request options.
    pub async fn create_with_options(
        &self,
        request: &CreateRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/create",
                    operation: "refunds.create",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a refund
    pub async fn cancel(&self, request: &CancelRefundRequest) -> Result<Refund> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a refund with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/cancel",
                    operation: "refunds.cancel",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Look up a refund
    pub async fn lookup(&self, request: &LookupRefundRequest) -> Result<Refund> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a refund with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupRefundRequest,
        options: RequestOptions,
    ) -> Result<Refund> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/lookup",
                    operation: "refunds.lookup",
                    field: Some("refund"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Page through refunds
    pub async fn page(&self, request: &PageRefundsRequest) -> Result<RefundPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }
    /// Page through refunds with per-request options.
    pub async fn page_with_options(
        &self,
        request: &PageRefundsRequest,
        options: RequestOptions,
    ) -> Result<RefundPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/refunds/page",
                    operation: "refunds.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

/// Operations for Inttegro schedules.
#[derive(Clone)]
pub struct Schedules {
    client: Client,
}

impl Schedules {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Look up a scheduled Chime
    pub async fn lookup(&self, request: &LookupScheduleRequest) -> Result<ScheduleDetail> {
        self.lookup_with_options(request, RequestOptions::default())
            .await
    }
    /// Look up a scheduled Chime with per-request options.
    pub async fn lookup_with_options(
        &self,
        request: &LookupScheduleRequest,
        options: RequestOptions,
    ) -> Result<ScheduleDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/schedules/lookup",
                    operation: "schedules.lookup",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }

    /// Cancel a scheduled Chime
    pub async fn cancel(&self, request: &CancelScheduleRequest) -> Result<ScheduleCancelDetail> {
        self.cancel_with_options(request, RequestOptions::default())
            .await
    }
    /// Cancel a scheduled Chime with per-request options.
    pub async fn cancel_with_options(
        &self,
        request: &CancelScheduleRequest,
        options: RequestOptions,
    ) -> Result<ScheduleCancelDetail> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/schedules/cancel",
                    operation: "schedules.cancel",
                    field: Some("scheduled_chime"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

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
