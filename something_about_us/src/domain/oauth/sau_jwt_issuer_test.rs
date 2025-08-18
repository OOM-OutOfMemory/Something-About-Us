use super::{KeyPair, SAUJwtIssuer};
use crate::domain::oauth::error::SAUOAuthDomainError;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

fn create_test_key_pair() -> KeyPair {
    // Ed25519 키 쌍 생성 (테스트용)
    let private_key_pem = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIJ+DYvh6SEqVTm50DFtMDoQikTmiCqirVv9mWG9qfSnF
-----END PRIVATE KEY-----";
    
    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAhSDwCYkwp1R0i33ctD73Wg2/Og+SorMWtOvI/PJJtEo=
-----END PUBLIC KEY-----";

    KeyPair {
        private_key: EncodingKey::from_ed_pem(private_key_pem.as_bytes()).unwrap(),
        public_key: DecodingKey::from_ed_pem(public_key_pem.as_bytes()).unwrap(),
        x: "hSDwCYkwp1R0i33ctD73Wg2/Og+SorMWtOvI/PJJtEo".to_string(),
    }
}

fn create_test_issuer() -> SAUJwtIssuer {
    let mut key_pairs = HashMap::new();
    let kid = Uuid::new_v4();
    key_pairs.insert(kid, create_test_key_pair());

    SAUJwtIssuer::new(
        "test-issuer".to_string(),
        "test-audience".to_string(),
        3600, // 1 hour
        key_pairs,
    )
}

#[test]
fn test_new_creates_valid_issuer() {
    let issuer = create_test_issuer();
    
    assert_eq!(issuer.iss, "test-issuer");
    assert_eq!(issuer.aud, "test-audience");
    assert_eq!(issuer.access_token_ttl, Duration::from_secs(3600));
    assert_eq!(issuer.header.alg, jsonwebtoken::Algorithm::EdDSA);
    assert!(!issuer.key_pair.is_empty());
}

#[test]
fn test_validate_success() {
    let issuer = create_test_issuer();
    assert!(issuer.validate().is_ok());
}

#[test]
fn test_validate_issuer_valid() {
    assert!(SAUJwtIssuer::validate_issuer("valid-issuer").is_ok());
    assert!(SAUJwtIssuer::validate_issuer("a").is_ok());
    assert!(SAUJwtIssuer::validate_issuer(&"x".repeat(50)).is_ok());
}

#[test]
fn test_validate_issuer_empty() {
    let result = SAUJwtIssuer::validate_issuer("");
    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::InvalidIssuer(msg) => assert_eq!(msg, ""),
        _ => panic!("Expected InvalidIssuer error"),
    }
}

#[test]
fn test_validate_issuer_too_long() {
    let long_issuer = "x".repeat(51);
    let result = SAUJwtIssuer::validate_issuer(&long_issuer);
    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::InvalidIssuer(msg) => assert_eq!(msg, long_issuer),
        _ => panic!("Expected InvalidIssuer error"),
    }
}

#[test]
fn test_validate_audience_valid() {
    assert!(SAUJwtIssuer::validate_audience("valid-audience").is_ok());
    assert!(SAUJwtIssuer::validate_audience("a").is_ok());
    assert!(SAUJwtIssuer::validate_audience(&"x".repeat(50)).is_ok());
}

#[test]
fn test_validate_audience_empty() {
    let result = SAUJwtIssuer::validate_audience("");
    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::InvalidAudience(msg) => assert_eq!(msg, ""),
        _ => panic!("Expected InvalidAudience error"),
    }
}

#[test]
fn test_validate_audience_too_long() {
    let long_audience = "x".repeat(51);
    let result = SAUJwtIssuer::validate_audience(&long_audience);
    assert!(result.is_err());
    match result.unwrap_err() {
        SAUOAuthDomainError::InvalidAudience(msg) => assert_eq!(msg, long_audience),
        _ => panic!("Expected InvalidAudience error"),
    }
}

#[test]
#[should_panic(expected = "validation error")]
fn test_new_with_invalid_issuer_panics() {
    let mut key_pairs = HashMap::new();
    let kid = Uuid::new_v4();
    key_pairs.insert(kid, create_test_key_pair());

    SAUJwtIssuer::new(
        "".to_string(), // invalid issuer
        "test-audience".to_string(),
        3600,
        key_pairs,
    );
}

#[test]
#[should_panic(expected = "validation error")]
fn test_new_with_invalid_audience_panics() {
    let mut key_pairs = HashMap::new();
    let kid = Uuid::new_v4();
    key_pairs.insert(kid, create_test_key_pair());

    SAUJwtIssuer::new(
        "test-issuer".to_string(),
        "".to_string(), // invalid audience
        3600,
        key_pairs,
    );
}

#[test]
fn test_clone() {
    let issuer = create_test_issuer();
    let cloned = issuer.clone();
    
    assert_eq!(issuer.iss, cloned.iss);
    assert_eq!(issuer.aud, cloned.aud);
    assert_eq!(issuer.access_token_ttl, cloned.access_token_ttl);
    assert_eq!(issuer.header.alg, cloned.header.alg);
}

#[test]
fn test_key_pair_structure() {
    let key_pair = create_test_key_pair();
    assert!(!key_pair.x.is_empty());
    // 키가 올바르게 생성되었는지 확인하기 위해 간단한 검증
    assert!(key_pair.x.len() > 10);
}

#[test]
fn test_multiple_key_pairs() {
    let mut key_pairs = HashMap::new();
    let kid1 = Uuid::new_v4();
    let kid2 = Uuid::new_v4();
    
    key_pairs.insert(kid1, create_test_key_pair());
    key_pairs.insert(kid2, create_test_key_pair());

    let issuer = SAUJwtIssuer::new(
        "test-issuer".to_string(),
        "test-audience".to_string(),
        3600,
        key_pairs,
    );

    assert_eq!(issuer.key_pair.len(), 2);
    assert!(issuer.key_pair.contains_key(&kid1));
    assert!(issuer.key_pair.contains_key(&kid2));
}

#[test]
fn test_access_token_ttl_conversion() {
    let mut key_pairs = HashMap::new();
    let kid = Uuid::new_v4();
    key_pairs.insert(kid, create_test_key_pair());

    let issuer = SAUJwtIssuer::new(
        "test-issuer".to_string(),
        "test-audience".to_string(),
        7200, // 2 hours in seconds
        key_pairs,
    );

    assert_eq!(issuer.access_token_ttl, Duration::from_secs(7200));
}