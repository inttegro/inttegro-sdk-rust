//! Children-policy API operations.

use super::ChildrenPolicy;
use crate::{Client, RequestOptions, RequestSpec, Result};

#[derive(Clone)]
pub struct ChildrenPolicies {
    client: Client,
}

impl ChildrenPolicies {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn lookup(&self) -> Result<ChildrenPolicy> {
        self.lookup_with_options(RequestOptions::default()).await
    }

    pub async fn lookup_with_options(&self, options: RequestOptions) -> Result<ChildrenPolicy> {
        let body = serde_json::json!({});
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
}
