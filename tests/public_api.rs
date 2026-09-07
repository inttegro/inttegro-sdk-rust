use inttegro::{Client, CreateOrderRequest, Order, RequestOptions};
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
