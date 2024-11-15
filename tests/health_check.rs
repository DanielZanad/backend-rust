use std::net::TcpListener;

use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addrs = spawn_app().await;
    spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", addrs))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert'
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    // Launch to server as background task
    // tokio::spawn returns a handle to the spawned future
    // but we have no use for it here, hance the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
