//! Operating-policy API operations.

use super::{CreateOperatingPolicyRequest, OperatingPolicy, UpdateOperatingPolicyRequest};
use crate::{Client, RequestOptions, RequestSpec, Result};

#[derive(Clone)]
pub struct OperatingPolicies {
    client: Client,
}

impl OperatingPolicies {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateOperatingPolicyRequest) -> Result<OperatingPolicy> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn create_with_options(
        &self,
        request: &CreateOperatingPolicyRequest,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        self.request(
            "/operating_policies/create",
            "operating_policies.create",
            request,
            options,
        )
        .await
    }

    pub async fn lookup(&self, operating_policy_id: impl Into<String>) -> Result<OperatingPolicy> {
        self.lookup_with_options(operating_policy_id, RequestOptions::default())
            .await
    }

    pub async fn lookup_with_options(
        &self,
        operating_policy_id: impl Into<String>,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        let body = serde_json::json!({"operating_policy_id": operating_policy_id.into()});
        self.request(
            "/operating_policies/lookup",
            "operating_policies.lookup",
            &body,
            options,
        )
        .await
    }

    pub async fn update(&self, request: &UpdateOperatingPolicyRequest) -> Result<OperatingPolicy> {
        self.update_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn update_with_options(
        &self,
        request: &UpdateOperatingPolicyRequest,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        self.request(
            "/operating_policies/update",
            "operating_policies.update",
            request,
            options,
        )
        .await
    }

    async fn request<B: serde::Serialize + ?Sized>(
        &self,
        path: &'static str,
        operation: &'static str,
        body: &B,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path,
                    operation,
                    field: Some("operating_policy"),
                    authenticated: true,
                },
                Some(body),
                options,
            )
            .await
    }
}
