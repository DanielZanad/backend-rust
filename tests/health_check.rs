use reqwest::Client;
use std::net::TcpListener;

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    let client = Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missingtheemail"),
        ("email=ursula_le_guin%40gmail.com", "missingthename"),
        ("", "missingbothnameandemail"),
        
    ];
    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            //Additional customised error message on test failure
            "The API did not fail with 400 BadRequest when the payload was {}.",
            error_message
        );
    }
}
