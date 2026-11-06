use inttegro::children_policy::{ChildrenPolicyPage, PageChildrenPoliciesRequest};
use inttegro::operating_policy::{OperatingPolicyPage, PageOperatingPoliciesRequest};

#[test]
fn policy_page_requests_and_responses_use_version_history_shape() {
    assert_eq!(
        serde_json::to_value(PageChildrenPoliciesRequest {
            number: Some(0),
            size: Some(20),
        })
        .unwrap(),
        serde_json::json!({"number": 0, "size": 20})
    );
    assert_eq!(
        serde_json::to_value(PageOperatingPoliciesRequest {
            operating_policy_id: "op_1".into(),
            number: None,
            size: Some(10),
        })
        .unwrap(),
        serde_json::json!({"operating_policy_id": "op_1", "size": 10})
    );

    let children_page: ChildrenPolicyPage = serde_json::from_value(serde_json::json!({
        "number": 0,
        "size": 20,
        "count": 1,
        "total": 2,
        "has_more": true,
        "versions": [children_policy(2)],
    }))
    .unwrap();
    let operating_page: OperatingPolicyPage = serde_json::from_value(serde_json::json!({
        "number": 0,
        "size": 20,
        "count": 1,
        "total": 1,
        "has_more": false,
        "versions": [operating_policy(1)],
    }))
    .unwrap();

    assert_eq!(children_page.versions[0].version, 2);
    assert!(children_page.has_more);
    assert_eq!(operating_page.versions[0].id, "op_1");
    assert!(!operating_page.has_more);
}

#[test]
fn policy_aging_specs_expose_id() {
    let children: inttegro::children_policy::ChildrenPolicyBTAgingSpec =
        serde_json::from_value(serde_json::json!({
            "id": "bt_aging_spec_immediate",
            "t_plus": "0h"
        }))
        .unwrap();
    let operating: inttegro::operating_policy::OperatingPolicyBTAgingSpec =
        serde_json::from_value(serde_json::json!({
            "id": "bt_aging_spec_168_hourly",
            "t_plus": "168h"
        }))
        .unwrap();

    assert_eq!(children.id, "bt_aging_spec_immediate");
    assert_eq!(operating.id, "bt_aging_spec_168_hourly");
}

fn children_policy(version: u64) -> serde_json::Value {
    serde_json::json!({
        "id": "cp_1",
        "version": version,
        "active_version": 2,
        "name": "Default",
        "capabilities": {},
        "created_at": "2026-11-05T10:00:00Z",
        "updated_at": "2026-11-05T10:00:00Z",
        "version_created_at": "2026-11-05T10:00:00Z"
    })
}

fn operating_policy(version: u64) -> serde_json::Value {
    serde_json::json!({
        "id": "op_1",
        "version": version,
        "active_version": 2,
        "name": "Retail",
        "capabilities": {},
        "children_policy_id": "cp_1",
        "children_policy_version": 1,
        "created_at": "2026-11-05T10:00:00Z",
        "updated_at": "2026-11-05T10:00:00Z",
        "version_created_at": "2026-11-05T10:00:00Z"
    })
}
