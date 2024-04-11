use reqwest::StatusCode;

use PKG_NAME::*;

use crate::helpers::TestApp;

#[tokio::test]
async fn healthcheck_works() {
    let state = AppState::new().unwrap();
    let app = TestApp::spawn(state).await;

    let response = app.get_healthcheck().await;

    assert_eq!(response.status(), StatusCode::OK);
}
