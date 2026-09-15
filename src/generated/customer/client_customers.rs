//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

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
