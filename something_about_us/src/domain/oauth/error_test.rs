use super::SAUOAuthDomainError;

#[test]
fn test_invalid_issuer_error_display() {
    let error = SAUOAuthDomainError::InvalidIssuer("test-issuer".to_string());
    assert_eq!(error.to_string(), "invalid issuer: test-issuer");
}

#[test]
fn test_invalid_audience_error_display() {
    let error = SAUOAuthDomainError::InvalidAudience("test-audience".to_string());
    assert_eq!(error.to_string(), "invalid audience: test-audience");
}

#[test]
fn test_invalid_url_error_display() {
    let error = SAUOAuthDomainError::InvalidUrl("invalid-url".to_string());
    assert_eq!(error.to_string(), "invalid url: invalid-url");
}

#[test]
fn test_login_failed_error_display() {
    let error = SAUOAuthDomainError::LoginFailed("connection timeout".to_string());
    assert_eq!(error.to_string(), "login failed: connection timeout");
}

#[test]
fn test_callback_failed_error_display() {
    let error = SAUOAuthDomainError::CallBackFailed("invalid code".to_string());
    assert_eq!(error.to_string(), "callback failed: invalid code");
}

#[test]
fn test_user_info_fetch_failed_error_display() {
    let error = SAUOAuthDomainError::UserInfoFetchFailed("unauthorized".to_string());
    assert_eq!(error.to_string(), "user info fetch failed : unauthorized");
}

#[test]
fn test_jwt_issue_failed_error_display() {
    let error = SAUOAuthDomainError::JwtIssueFailed("key not found".to_string());
    assert_eq!(error.to_string(), "jwt issue failed: key not found");
}

#[test]
fn test_error_debug_format() {
    let error = SAUOAuthDomainError::InvalidIssuer("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("InvalidIssuer"));
    assert!(debug_str.contains("test"));
}

#[test]
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SAUOAuthDomainError>();
}