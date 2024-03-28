use reqwest::StatusCode;

use PKG_NAME::*;

#[tokio::test]
async fn healthcheck_works() {
    let state = AppState::default();
    let addr = spawn_test_app(state);

    let response = reqwest::get(format!("http://{addr}/random_number"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
