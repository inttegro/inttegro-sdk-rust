use inttegro::{Client, CreateOrderRequest, CustomData, CustomDataPatch, Order, RequestOptions};
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
