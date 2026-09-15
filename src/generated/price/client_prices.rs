//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

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
