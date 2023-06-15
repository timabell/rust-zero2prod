#[tokio::test]
async fn health_check_returns_http_ok(){
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("bang");
    assert!(response.status().is_success());
    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let server = zero2prod::build_server().expect("server build failed");
    tokio::spawn(server);
}
