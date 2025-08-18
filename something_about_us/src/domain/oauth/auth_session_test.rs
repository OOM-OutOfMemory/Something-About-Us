use super::{AuthSession, AUTH_SESSION_COOKIE_NAME, AUTH_HTTP_AGENT_NAME};
use uuid::Uuid;

fn create_test_auth_session() -> AuthSession {
    AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: "test-pkce-verifier-123456789".to_string(),
        csrf_token: "test-csrf-token-987654321".to_string(),
    }
}

#[test]
fn test_auth_session_creation() {
    let session = create_test_auth_session();
    
    assert!(!session.pkce_verifier.is_empty());
    assert!(!session.csrf_token.is_empty());
    assert_ne!(session.id, Uuid::nil());
}

#[test]
fn test_auth_session_serialization() {
    let session = create_test_auth_session();
    let serialized = sonic_rs::to_string(&session).expect("Failed to serialize");
    
    assert!(serialized.contains("id"));
    assert!(serialized.contains("pkce_verifier"));
    assert!(serialized.contains("csrf_token"));
    assert!(serialized.contains("test-pkce-verifier"));
    assert!(serialized.contains("test-csrf-token"));
}

#[test]
fn test_auth_session_deserialization() {
    let original = create_test_auth_session();
    let serialized = sonic_rs::to_string(&original).expect("Failed to serialize");
    let deserialized: AuthSession = sonic_rs::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.pkce_verifier, deserialized.pkce_verifier);
    assert_eq!(original.csrf_token, deserialized.csrf_token);
}

#[test]
fn test_auth_session_round_trip() {
    let session1 = AuthSession {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        pkce_verifier: "verifier123".to_string(),
        csrf_token: "token456".to_string(),
    };
    
    let json = sonic_rs::to_string(&session1).unwrap();
    let session2: AuthSession = sonic_rs::from_str(&json).unwrap();
    
    assert_eq!(session1.id, session2.id);
    assert_eq!(session1.pkce_verifier, session2.pkce_verifier);
    assert_eq!(session1.csrf_token, session2.csrf_token);
}

#[test]
fn test_constants() {
    assert_eq!(AUTH_SESSION_COOKIE_NAME, "something_about_us_auth_session");
    assert_eq!(AUTH_HTTP_AGENT_NAME, "SomethingAboutUs");
    
    // Test that constants are not empty
    assert!(!AUTH_SESSION_COOKIE_NAME.is_empty());
    assert!(!AUTH_HTTP_AGENT_NAME.is_empty());
    
    // Test reasonable length constraints
    assert!(AUTH_SESSION_COOKIE_NAME.len() < 100);
    assert!(AUTH_HTTP_AGENT_NAME.len() < 50);
}

#[test]
fn test_auth_session_with_empty_strings() {
    let session = AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: String::new(),
        csrf_token: String::new(),
    };
    
    // Should still serialize/deserialize correctly
    let serialized = sonic_rs::to_string(&session).expect("Failed to serialize");
    let deserialized: AuthSession = sonic_rs::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(session.id, deserialized.id);
    assert_eq!(session.pkce_verifier, deserialized.pkce_verifier);
    assert_eq!(session.csrf_token, deserialized.csrf_token);
    assert!(deserialized.pkce_verifier.is_empty());
    assert!(deserialized.csrf_token.is_empty());
}

#[test]
fn test_auth_session_with_special_characters() {
    let session = AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: "verifier-with-special-chars!@#$%^&*()".to_string(),
        csrf_token: "token_with_underscores_and_numbers_123".to_string(),
    };
    
    let serialized = sonic_rs::to_string(&session).expect("Failed to serialize");
    let deserialized: AuthSession = sonic_rs::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(session.pkce_verifier, deserialized.pkce_verifier);
    assert_eq!(session.csrf_token, deserialized.csrf_token);
}

#[test]
fn test_auth_session_with_long_strings() {
    let long_verifier = "a".repeat(1000);
    let long_token = "b".repeat(500);
    
    let session = AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: long_verifier.clone(),
        csrf_token: long_token.clone(),
    };
    
    let serialized = sonic_rs::to_string(&session).expect("Failed to serialize");
    let deserialized: AuthSession = sonic_rs::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(session.pkce_verifier, deserialized.pkce_verifier);
    assert_eq!(session.csrf_token, deserialized.csrf_token);
    assert_eq!(deserialized.pkce_verifier.len(), 1000);
    assert_eq!(deserialized.csrf_token.len(), 500);
}

#[test]
fn test_unique_session_ids() {
    let session1 = AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: "verifier1".to_string(),
        csrf_token: "token1".to_string(),
    };
    
    let session2 = AuthSession {
        id: Uuid::new_v4(),
        pkce_verifier: "verifier2".to_string(),
        csrf_token: "token2".to_string(),
    };
    
    // UUIDs should be different
    assert_ne!(session1.id, session2.id);
}