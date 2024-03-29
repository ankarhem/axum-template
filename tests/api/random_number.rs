use std::sync::Arc;

use error_stack::Report;
use reqwest::StatusCode;

use PKG_NAME::{clients::*, spawn_test_app, AppState};

#[tokio::test]
async fn random_number_multiplies_by_100() {
    let mut mock_service = MockRandomNumberService::new();
    mock_service.expect_get_random_number().returning(|| Ok(42));

    let state = AppState::default().replace_random_number_service(Arc::new(mock_service));
    let addr = spawn_test_app(state);

    let response = reqwest::get(format!("http://{addr}/random_number"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    assert_eq!(body, "4200");
}

#[tokio::test]
async fn sends_500_when_underlying_service_fails() {
    let mut mock_service = MockRandomNumberService::new();
    mock_service
        .expect_get_random_number()
        .returning(|| Err(Report::new(RandomNumberServiceError)));

    let state = AppState::default().replace_random_number_service(Arc::new(mock_service));
    let addr = spawn_test_app(state);

    let response = reqwest::get(format!("http://{addr}/random_number"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn handles_overflow_with_500() {
    let mut mock_service = MockRandomNumberService::new();
    mock_service
        .expect_get_random_number()
        .returning(|| Ok(u16::MAX));

    let state = AppState::default().replace_random_number_service(Arc::new(mock_service));
    let addr = spawn_test_app(state);

    let response = reqwest::get(format!("http://{addr}/random_number"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
