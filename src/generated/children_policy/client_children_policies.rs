//! Children-policy API operations.

use super::{ChildrenPolicy, ChildrenPolicyPage, PageChildrenPoliciesRequest};
use crate::{Client, InttegroError, RequestOptions, RequestSpec, Result};
use serde::Serialize;

#[derive(Serialize)]
struct LookupChildrenPolicyBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u64>,
}

#[derive(Clone)]
pub struct ChildrenPolicies {
    client: Client,
}

impl ChildrenPolicies {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve the latest active children-policy version.
    pub async fn lookup(&self) -> Result<ChildrenPolicy> {
        self.lookup_with_options(RequestOptions::default()).await
    }

    pub async fn lookup_with_options(&self, options: RequestOptions) -> Result<ChildrenPolicy> {
        self.lookup_request(None, options).await
    }

    /// Retrieve an exact immutable children-policy version.
    pub async fn lookup_version(&self, version: u64) -> Result<ChildrenPolicy> {
        self.lookup_version_with_options(version, RequestOptions::default())
            .await
    }

    pub async fn lookup_version_with_options(
        &self,
        version: u64,
        options: RequestOptions,
    ) -> Result<ChildrenPolicy> {
        validate_version(Some(version))?;
        self.lookup_request(Some(version), options).await
    }

    async fn lookup_request(
        &self,
        version: Option<u64>,
        options: RequestOptions,
    ) -> Result<ChildrenPolicy> {
        let body = LookupChildrenPolicyBody { version };
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/children_policies/lookup",
                    operation: "children_policies.lookup",
                    field: Some("children_policy"),
                    authenticated: true,
                },
                Some(&body),
                options,
            )
            .await
    }

    /// List immutable children-policy versions, newest first.
    pub async fn page(&self, request: &PageChildrenPoliciesRequest) -> Result<ChildrenPolicyPage> {
        self.page_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn page_with_options(
        &self,
        request: &PageChildrenPoliciesRequest,
        options: RequestOptions,
    ) -> Result<ChildrenPolicyPage> {
        self.client
            .request_resource(
                RequestSpec {
                    method: "POST",
                    path: "/children_policies/page",
                    operation: "children_policies.page",
                    field: Some("page"),
                    authenticated: true,
                },
                Some(request),
                options,
            )
            .await
    }
}

fn validate_version(version: Option<u64>) -> Result<()> {
    if version == Some(0) {
        return Err(InttegroError::Configuration(
            "version must be at least 1".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::LookupChildrenPolicyBody;

    #[test]
    fn lookup_body_omits_or_serializes_exact_version() {
        assert_eq!(
            serde_json::to_value(LookupChildrenPolicyBody { version: None }).unwrap(),
            serde_json::json!({})
        );
        assert_eq!(
            serde_json::to_value(LookupChildrenPolicyBody { version: Some(2) }).unwrap(),
            serde_json::json!({"version": 2})
        );
    }
}
