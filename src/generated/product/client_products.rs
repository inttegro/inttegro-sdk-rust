//! Typed Inttegro API resource operations.

#[allow(unused_imports)]
use crate::generated::*;
#[allow(unused_imports)]
use crate::{
    BalanceSnapshot, Client, CountrySpecifications, CreateFileRequest, FileDownload,
    FulfillUploadRequest, OpenFileLinkRequest, RequestOptions, RequestSpec, Result,
};

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
