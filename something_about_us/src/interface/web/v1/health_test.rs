#[cfg(test)]
mod tests {
    use crate::interface::web_test::test_make_route;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_health_check() {
        let server = axum_test::TestServer::new(test_make_route().await).unwrap();
        let response = server.get("/v1/heartbeat").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let body = response.text();
        assert_eq!(body, "Ok - Something About Us");
    }
}
