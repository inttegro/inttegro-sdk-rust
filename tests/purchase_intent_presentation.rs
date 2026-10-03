use inttegro::purchase_intent::{
    CreatePurchaseIntentPresentation, CreatePurchaseIntentPresentationBuyPage,
    CreatePurchaseIntentPresentationBuyPageText, CreatePurchaseIntentRequest,
    CreatePurchaseIntentRequestQuantity, PurchaseIntent, UpdatePurchaseIntentPresentation,
    UpdatePurchaseIntentPresentationBuyPage, UpdatePurchaseIntentPresentationBuyPageText,
    UpdatePurchaseIntentRequest,
};

#[test]
fn purchase_intent_presentation_serializes_and_decodes() {
    let request = CreatePurchaseIntentRequest {
        product: None,
        product_id: Some("prod_123".into()),
        price: None,
        price_id: Some("pr_123".into()),
        usage: None,
        expires_at: None,
        presentation: Some(CreatePurchaseIntentPresentation {
            buy_page: CreatePurchaseIntentPresentationBuyPage {
                text: CreatePurchaseIntentPresentationBuyPageText {
                    checkout_section_title: None,
                    amount_field_label: Some("Your contribution".into()),
                    primary_action_label: None,
                },
            },
        }),
        quantity: CreatePurchaseIntentRequestQuantity { max: None, min: 1 },
    };
    let encoded = serde_json::to_value(request).expect("serialize request");
    assert_eq!(
        encoded["presentation"]["buy_page"]["text"]["amount_field_label"],
        "Your contribution"
    );

    let intent: PurchaseIntent = serde_json::from_value(serde_json::json!({
        "allow_variants": false,
        "created_at": "2026-09-09T12:00:00Z",
        "id": "sale_123",
        "presentation": {"buy_page": {"text": {"checkout_section_title": "Support this cause"}}},
        "quantity": {"min": 1},
        "status": "active",
        "usage": {"multi_use": true}
    }))
    .expect("decode purchase intent");
    assert_eq!(
        intent
            .presentation
            .expect("presentation")
            .buy_page
            .expect("buy page")
            .text
            .expect("text")
            .checkout_section_title
            .as_deref(),
        Some("Support this cause")
    );

    let update = UpdatePurchaseIntentRequest {
        expires_at: None,
        id: Some("sale_123".into()),
        quantity: None,
        purchase_intent_id: None,
        reactivate: None,
        presentation: Some(UpdatePurchaseIntentPresentation {
            buy_page: UpdatePurchaseIntentPresentationBuyPage {
                text: UpdatePurchaseIntentPresentationBuyPageText {
                    checkout_section_title: Some(Some("Contribute now".into())),
                    amount_field_label: Some(None),
                    primary_action_label: None,
                },
            },
        }),
    };
    let encoded = serde_json::to_value(update).expect("serialize update");
    assert_eq!(
        encoded["presentation"]["buy_page"]["text"]["checkout_section_title"],
        "Contribute now"
    );
    assert!(encoded["presentation"]["buy_page"]["text"]["amount_field_label"].is_null());
    assert!(
        encoded["presentation"]["buy_page"]["text"]
            .get("primary_action_label")
            .is_none()
    );
}
