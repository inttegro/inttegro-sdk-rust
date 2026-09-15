//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

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
}
