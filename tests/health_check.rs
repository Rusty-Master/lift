use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn route_create_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "start_lat=1&start_lon=2&end_lat=3&end_lon=4";
    let response = client
        .post(&format!("{}/v1/route/create", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn route_create_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("start_lon=2&end_lat=3&end_lon=4", "missing 1"),
        ("start_lat=1&end_lat=3&end_lon=4", "missing 2"),
        ("start_lat=1&start_lon=2&end_lon=4", "missing 3"),
        ("start_lat=1&start_lon=2&end_lat=3", "missing 4"),
        ("", "missing all"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/v1/route/create", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = lift::run(listener).expect("Failed to bind address");
    actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
