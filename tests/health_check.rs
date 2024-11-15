#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert'
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    // Launch to server as background task
    // tokio::spawn returns a handle to the spawned future
    // but we have no use for it here, hance the non-binding let
    let _ = tokio::spawn(server);
}
