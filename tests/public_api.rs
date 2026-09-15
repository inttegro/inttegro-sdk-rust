use inttegro::money::Currency;
use inttegro::order::{CreateOrderRequest, Order};
use inttegro::payout::PayoutSettingsMutation;
use inttegro::purchase_intent::PurchaseIntent;
use inttegro::refund::{Refund, RefundSettlement, RefundSettlementPaymentMethod};
use inttegro::{BalanceSnapshot, Client, CustomData, CustomDataPatch, RequestOptions};
use std::io::{Read, Write};
use std::net::TcpListener;

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn public_types_are_concurrency_safe() {
    assert_send_sync::<Client>();
    assert_send_sync::<Order>();
    assert_send_sync::<CreateOrderRequest>();
    let _ = RequestOptions::default();
}

#[test]
fn resource_modules_expose_canonical_models_and_clients() {
    let _: Option<inttegro::order::Order> = None;
    let _: Option<inttegro::order::Orders> = None;
    let _: Option<inttegro::payment_method::PaymentMethod> = None;
    let _: Option<inttegro::payment_method::PaymentMethods> = None;
    let _: Option<inttegro::refund::Refund> = None;
    let _: Option<inttegro::country::CountrySpecification> = None;
}

#[test]
fn currency_constants_preserve_iso_codes_and_lowercase_wire_values() {
    let currencies = [
        (Currency::GHS, "ghs"),
        (Currency::USD, "usd"),
        (Currency::GBP, "gbp"),
        (Currency::EUR, "eur"),
        (Currency::CNY, "cny"),
    ];

    for (currency, wire_value) in currencies {
        assert_eq!(
            serde_json::to_value(currency).unwrap(),
            serde_json::json!(wire_value)
        );
        assert_eq!(
            serde_json::from_value::<Currency>(serde_json::json!(wire_value)).unwrap(),
            currency
        );
    }
}

#[test]
fn semantic_collections_control_custom_data_mutation() {
    let mut data = CustomData::new();
    data.insert("order", "first").unwrap();
    assert_eq!(data.get("order"), Some("first"));

    let oversized_key = "x".repeat(257);
    assert!(data.insert(oversized_key, "invalid").is_err());
    assert_eq!(data.len(), 1, "failed mutations must be rolled back");

    let mut patch = CustomDataPatch::new();
    patch.set("campaign", "winter").unwrap();
    patch.unset("legacy").unwrap();
    assert_eq!(
        serde_json::to_value(patch).unwrap(),
        serde_json::json!({"campaign": "winter", "legacy": null})
    );
}

#[test]
fn balance_snapshot_exposes_ghs_statically() {
    let balance: BalanceSnapshot = serde_json::from_value(serde_json::json!({
        "ghs": {
            "available": {"amount": 1000},
            "includes_transactions_before": "2026-09-09T12:00:00Z",
            "pending": {"amount": 200},
            "refund": {"amount": 50},
            "reserved": {"amount": 100}
        }
    }))
    .unwrap();

    assert_eq!(balance.ghs.available.amount, 1_000);
    assert_eq!(
        balance.ghs.includes_transactions_before.to_string(),
        "2026-09-09T12:00:00Z"
    );
    assert!(
        serde_json::from_value::<BalanceSnapshot>(serde_json::json!({
            "ghs": {
                "available": {"amount": 1000},
                "includes_transactions_before": "2026-09-09T12:00:00",
                "pending": {"amount": 200},
                "refund": {"amount": 50},
                "reserved": {"amount": 100}
            }
        }))
        .is_err()
    );
}

#[test]
fn purchase_intent_exposes_nested_response_types() {
    let intent: PurchaseIntent = serde_json::from_value(serde_json::json!({
        "allow_variants": false,
        "created_at": "2026-09-09T12:00:00Z",
        "id": "sale_123",
        "merchant": {"organization_name": "Tea House Ltd"},
        "product": {
            "active": true,
            "created_at": "2026-09-09T11:00:00Z",
            "dimensions": {"digital": {"bytes": 1024}},
            "id": "prod_123",
            "name": "Tea guide",
            "type": "digital"
        },
        "quantity": {"min": 1},
        "status": "active",
        "usage": {
            "order": {"created_at": "2026-09-09T12:02:00Z", "id": "or_123"},
            "single_use": true
        }
    }))
    .unwrap();

    assert!(intent.is_active());
    assert!(intent.is_single_use());
    assert_eq!(intent.used_order_id(), Some("or_123"));

    assert_eq!(
        intent.merchant.unwrap().organization_name.as_deref(),
        Some("Tea House Ltd")
    );
    assert_eq!(
        intent
            .product
            .unwrap()
            .dimensions
            .unwrap()
            .digital
            .unwrap()
            .bytes,
        Some(1_024.0)
    );
    assert_eq!(intent.usage.order.unwrap().id, "or_123");
}

#[test]
fn payout_settings_expose_known_destinations_statically() {
    let settings: PayoutSettingsMutation = serde_json::from_value(serde_json::json!({
        "destinations": {"ghs": "fa_123"},
        "fx_enabled": true,
        "id": "settings_123"
    }))
    .unwrap();

    let destinations = settings.destinations.unwrap();
    assert_eq!(destinations.ghs.as_deref(), Some("fa_123"));
    assert_eq!(settings.fx_enabled, Some(true));
}

#[test]
fn refund_settlement_is_discriminated_and_masked() {
    let refund: Refund = serde_json::from_value(serde_json::json!({
        "created_at": "2026-09-09T12:00:00Z",
        "id": "rf_123",
        "line_items": [],
        "order_id": "or_123",
        "reason": "requested_by_customer",
        "settlement": {
            "type": "payment_method",
            "payment_method": {
                "id": "pm_123",
                "type": "bank_account",
                "bank_account": {
                    "type": "ghana_bank_account",
                    "ghana_bank_account": {
                        "account_number": "****1234",
                        "last4": "1234"
                    }
                }
            }
        },
        "status": "pending",
        "total": {"currency": "ghs", "value": 1000}
    }))
    .unwrap();

    let RefundSettlement::PaymentMethod { payment_method } = refund.settlement else {
        panic!("expected payment-method settlement");
    };
    let RefundSettlementPaymentMethod::BankAccount { bank_account, .. } = payment_method else {
        panic!("expected bank-account snapshot");
    };
    let inttegro::refund::RefundSettlementBankAccount::GhanaBankAccount { ghana_bank_account } =
        bank_account;
    assert_eq!(ghana_bank_account.account_number, "****1234");

    assert!(
        serde_json::from_value::<RefundSettlement>(serde_json::json!({
            "type": "offline",
            "payment_method": {"id": "pm_123", "type": "mobile_money"}
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<RefundSettlement>(serde_json::json!({
            "type": "payment_method"
        }))
        .is_err()
    );
}

#[test]
fn resources_answer_protocol_questions() {
    let payment: inttegro::payment::Payment = serde_json::from_value(serde_json::json!({
        "amount": {"currency": "ghs", "value": 1000},
        "id": "py_123",
        "initiated_at": "2026-09-09T12:00:00Z",
        "next_action": {"type": "redirect"},
        "statement_descriptor": "INTTEGRO",
        "status": "requires_action"
    }))
    .unwrap();
    assert!(payment.requires_action());
    assert!(!payment.is_terminal());
    assert!(payment.required_action().is_some());

    let product: inttegro::product::Product = serde_json::from_value(serde_json::json!({
        "active": true,
        "created_at": "2026-09-09T12:00:00Z",
        "id": "prod_123",
        "name": "Tea guide",
        "published_at": "2026-09-09T12:00:00Z",
        "type": "digital"
    }))
    .unwrap();
    assert!(product.is_published());
    assert!(product.was_ever_published());

    let method: inttegro::payment_method::PaymentMethod =
        serde_json::from_value(serde_json::json!({
            "active": true,
            "created_at": "2026-09-09T12:00:00Z",
            "customer_id": "cu_123",
            "fingerprint": "fp_123",
            "id": "pm_123",
            "type": "mobile_money",
            "verified_at": "2026-09-09T12:00:00Z"
        }))
        .unwrap();
    assert!(method.is_verified());
    assert!(method.is_reusable());
}

#[tokio::test]
async fn unwraps_the_wire_envelope_into_a_domain_value() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request).unwrap();
        let body = r#"{"app":{"id":"app_test","name":"Test","created_at":"2026-09-07T00:00:00Z"}}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: {}
Connection: close

{}",
            body.len(),
            body
        )
        .unwrap();
    });
    let client = Client::builder("sk_test_example")
        .base_url(format!("http://{address}"))
        .build()
        .unwrap();
    let app = client.apps().lookup().await.unwrap();
    assert_eq!(app.id, "app_test");
    server.join().unwrap();
}
