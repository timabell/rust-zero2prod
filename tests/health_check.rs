use std::net::TcpListener;

#[tokio::test]
async fn health_check_returns_http_ok() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}/health_check", address);
    let response = client.get(endpoint).send().await.expect("bang");
    assert!(response.status().is_success());
    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("failed to bind to port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::build_server(listener).expect("server build failed");
    tokio::spawn(server);
    format!("http://localhost:{}", port)
}
