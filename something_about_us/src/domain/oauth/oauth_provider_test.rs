use super::OAuthRequest;
use crate::domain::oauth::{
    auth_session::AuthSession, error::SAUOAuthDomainError, sau_jwt::OAuthAccessToken,
};
use url::Url;
use uuid::Uuid;

// Mock implementation for testing
struct MockOAuthProvider {
    should_fail_login: bool,
    should_fail_callback: bool,
    should_fail_user_id: bool,
}

impl MockOAuthProvider {
    fn new() -> Self {
        Self {
            should_fail_login: false,
            should_fail_callback: false,
            should_fail_user_id: false,
        }
    }

    fn with_login_failure() -> Self {
        Self {
            should_fail_login: true,
            should_fail_callback: false,
            should_fail_user_id: false,
        }
    }

    fn with_callback_failure() -> Self {
        Self {
            should_fail_login: false,
            should_fail_callback: true,
            should_fail_user_id: false,
        }
    }

    fn with_user_id_failure() -> Self {
        Self {
            should_fail_login: false,
            should_fail_callback: false,
            should_fail_user_id: true,
        }
    }
}

#[async_trait::async_trait]
impl OAuthRequest for MockOAuthProvider {
    async fn login(&self) -> Result<(Url, AuthSession), SAUOAuthDomainError> {
        if self.should_fail_login {
            return Err(SAUOAuthDomainError::LoginFailed(
                "Mock login failure".to_string(),
            ));
        }

        let url =
            Url::parse("https://example.com/oauth/authorize?client_id=test&redirect_uri=test")
                .map_err(|e| SAUOAuthDomainError::InvalidUrl(e.to_string()))?;

        let session = AuthSession {
            id: Uuid::new_v4(),
            pkce_verifier: "mock-pkce-verifier".to_string(),
            csrf_token: "mock-csrf-token".to_string(),
        };

        Ok((url, session))
    }

    async fn callback(
        &self,
        code: String,
        pkce_verifier: String,
    ) -> Result<OAuthAccessToken, SAUOAuthDomainError> {
        if self.should_fail_callback {
            return Err(SAUOAuthDomainError::CallBackFailed(
                "Mock callback failure".to_string(),
            ));
        }

        if code.is_empty() {
            return Err(SAUOAuthDomainError::CallBackFailed(
                "Empty authorization code".to_string(),
            ));
        }

        if pkce_verifier.is_empty() {
            return Err(SAUOAuthDomainError::CallBackFailed(
                "Empty PKCE verifier".to_string(),
            ));
        }

        Ok("mock-access-token".to_string())
    }

    async fn get_user_id(
        &self,
        access_token: OAuthAccessToken,
    ) -> Result<String, SAUOAuthDomainError> {
        if self.should_fail_user_id {
            return Err(SAUOAuthDomainError::UserInfoFetchFailed(
                "Mock user info failure".to_string(),
            ));
        }

        if access_token.is_empty() {
            return Err(SAUOAuthDomainError::UserInfoFetchFailed(
                "Empty access token".to_string(),
            ));
        }

        Ok("mock-user-id".to_string())
    }
}

#[tokio::test]
async fn test_oauth_login_success() {
    let provider = MockOAuthProvider::new();
    let result = provider.login().await;

    assert!(result.is_ok());
    let (url, session) = result.unwrap();

    assert_eq!(url.scheme(), "https");
    assert_eq!(url.host_str(), Some("example.com"));
    assert!(url.path().contains("oauth"));
    assert!(!session.pkce_verifier.is_empty());
    assert!(!session.csrf_token.is_empty());
}

#[tokio::test]
async fn test_oauth_login_failure() {
    let provider = MockOAuthProvider::with_login_failure();
    let result = provider.login().await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::LoginFailed(msg) => {
            assert_eq!(msg, "Mock login failure");
        }
        _ => panic!("Expected LoginFailed error"),
    }
}

#[tokio::test]
async fn test_oauth_callback_success() {
    let provider = MockOAuthProvider::new();
    let result = provider
        .callback(
            "test-auth-code".to_string(),
            "test-pkce-verifier".to_string(),
        )
        .await;

    assert!(result.is_ok());
    let access_token = result.unwrap();
    assert_eq!(access_token, "mock-access-token");
}

#[tokio::test]
async fn test_oauth_callback_failure() {
    let provider = MockOAuthProvider::with_callback_failure();
    let result = provider
        .callback(
            "test-auth-code".to_string(),
            "test-pkce-verifier".to_string(),
        )
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::CallBackFailed(msg) => {
            assert_eq!(msg, "Mock callback failure");
        }
        _ => panic!("Expected CallBackFailed error"),
    }
}

#[tokio::test]
async fn test_oauth_callback_empty_code() {
    let provider = MockOAuthProvider::new();
    let result = provider
        .callback(String::new(), "test-pkce-verifier".to_string())
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::CallBackFailed(msg) => {
            assert_eq!(msg, "Empty authorization code");
        }
        _ => panic!("Expected CallBackFailed error"),
    }
}

#[tokio::test]
async fn test_oauth_callback_empty_verifier() {
    let provider = MockOAuthProvider::new();
    let result = provider
        .callback("test-auth-code".to_string(), String::new())
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::CallBackFailed(msg) => {
            assert_eq!(msg, "Empty PKCE verifier");
        }
        _ => panic!("Expected CallBackFailed error"),
    }
}

#[tokio::test]
async fn test_get_user_id_success() {
    let provider = MockOAuthProvider::new();
    let result = provider.get_user_id("test-access-token".to_string()).await;

    assert!(result.is_ok());
    let user_id = result.unwrap();
    assert_eq!(user_id, "mock-user-id");
}

#[tokio::test]
async fn test_get_user_id_failure() {
    let provider = MockOAuthProvider::with_user_id_failure();
    let result = provider.get_user_id("test-access-token".to_string()).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::UserInfoFetchFailed(msg) => {
            assert_eq!(msg, "Mock user info failure");
        }
        _ => panic!("Expected UserInfoFetchFailed error"),
    }
}

#[tokio::test]
async fn test_get_user_id_empty_token() {
    let provider = MockOAuthProvider::new();
    let result = provider.get_user_id(String::new()).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::UserInfoFetchFailed(msg) => {
            assert_eq!(msg, "Empty access token");
        }
        _ => panic!("Expected UserInfoFetchFailed error"),
    }
}

#[tokio::test]
async fn test_oauth_flow_integration() {
    let provider = MockOAuthProvider::new();

    // Step 1: Login
    let (url, session) = provider.login().await.expect("Login should succeed");
    assert!(url.as_str().contains("oauth"));

    // Step 2: Callback
    let access_token = provider
        .callback("auth-code".to_string(), session.pkce_verifier)
        .await
        .expect("Callback should succeed");

    // Step 3: Get user ID
    let user_id = provider
        .get_user_id(access_token)
        .await
        .expect("Get user ID should succeed");
    assert_eq!(user_id, "mock-user-id");
}

#[test]
fn test_trait_object_compatibility() {
    let provider: Box<dyn OAuthRequest> = Box::new(MockOAuthProvider::new());
    // This test ensures the trait can be used as a trait object
    assert!(!std::ptr::addr_of!(*provider).is_null());
}
