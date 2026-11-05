//! Operating-policy API operations.

use super::{
    CreateOperatingPolicyRequest, OperatingPolicy, OperatingPolicyPage,
    PageOperatingPoliciesRequest, UpdateOperatingPolicyRequest,
};
use crate::{Client, InttegroError, RequestOptions, RequestSpec, Result};
use serde::Serialize;

#[derive(Serialize)]
struct LookupOperatingPolicyBody<'a> {
    operating_policy_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u64>,
}

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

    /// Retrieve the latest active operating-policy version.
    pub async fn lookup(&self, operating_policy_id: impl Into<String>) -> Result<OperatingPolicy> {
        self.lookup_with_options(operating_policy_id, RequestOptions::default())
            .await
    }

    pub async fn lookup_with_options(
        &self,
        operating_policy_id: impl Into<String>,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        let operating_policy_id = operating_policy_id.into();
        self.lookup_request(&operating_policy_id, None, options)
            .await
    }

    /// Retrieve an exact immutable operating-policy version.
    pub async fn lookup_version(
        &self,
        operating_policy_id: impl Into<String>,
        version: u64,
    ) -> Result<OperatingPolicy> {
        self.lookup_version_with_options(operating_policy_id, version, RequestOptions::default())
            .await
    }

    pub async fn lookup_version_with_options(
        &self,
        operating_policy_id: impl Into<String>,
        version: u64,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        let operating_policy_id = operating_policy_id.into();
        self.lookup_request(&operating_policy_id, Some(version), options)
            .await
    }

    async fn lookup_request(
        &self,
        operating_policy_id: &str,
        version: Option<u64>,
        options: RequestOptions,
    ) -> Result<OperatingPolicy> {
        validate_request(operating_policy_id, version)?;
        let body = LookupOperatingPolicyBody {
            operating_policy_id,
            version,
        };
        self.request(
            "/operating_policies/lookup",
            "operating_policies.lookup",
            &body,
            options,
        )
        .await
    }

    /// List immutable versions of one operating policy, newest first.
    pub async fn page(
        &self,
        request: &PageOperatingPoliciesRequest,
    ) -> Result<OperatingPolicyPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn page_with_options(
        &self,
        request: &PageOperatingPoliciesRequest,
        options: RequestOptions,
    ) -> Result<OperatingPolicyPage> {
        validate_request(&request.operating_policy_id, None)?;
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/operating_policies/page",
                    operation: "operating_policies.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
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

fn validate_request(operating_policy_id: &str, version: Option<u64>) -> Result<()> {
    if operating_policy_id.trim().is_empty() {
        return Err(InttegroError::Configuration(
            "operating_policy_id is required".into(),
        ));
    }
    if version == Some(0) {
        return Err(InttegroError::Configuration(
            "version must be at least 1".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::LookupOperatingPolicyBody;

    #[test]
    fn lookup_body_omits_or_serializes_exact_version() {
        assert_eq!(
            serde_json::to_value(LookupOperatingPolicyBody {
                operating_policy_id: "op_1",
                version: None,
            })
            .unwrap(),
            serde_json::json!({"operating_policy_id": "op_1"})
        );
        assert_eq!(
            serde_json::to_value(LookupOperatingPolicyBody {
                operating_policy_id: "op_1",
                version: Some(3),
            })
            .unwrap(),
            serde_json::json!({"operating_policy_id": "op_1", "version": 3})
        );
    }
}
