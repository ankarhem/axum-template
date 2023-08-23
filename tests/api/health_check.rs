use reqwest::StatusCode;

use PKG_NAME::*;

#[tokio::test]
async fn healthcheck_works() {
    let addr = spawn_app();

    let response = reqwest::get(format!("http://{addr}/__healthcheck"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
