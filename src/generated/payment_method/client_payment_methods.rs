//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

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
