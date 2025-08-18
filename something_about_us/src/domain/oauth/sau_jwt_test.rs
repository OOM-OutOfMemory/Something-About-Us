use super::{OAuthAccessToken, SAUClaims, SAUJwt};
use chrono::Utc;
use uuid::Uuid;

fn create_test_claims() -> SAUClaims {
    let now = Utc::now().timestamp();
    SAUClaims {
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        sub: Uuid::new_v4(),
        exp: now + 3600, // 1 hour from now
        jti: Uuid::new_v4(),
        iat: now,
        nbf: now,
    }
}

#[test]
fn test_sau_claims_creation() {
    let claims = create_test_claims();
    
    assert_eq!(claims.aud, "test-audience");
    assert_eq!(claims.iss, "test-issuer");
    assert!(claims.exp > claims.iat);
    assert_eq!(claims.iat, claims.nbf);
}

#[test]
fn test_sau_claims_serialization() {
    let claims = create_test_claims();
    let serialized = sonic_rs::to_string(&claims).expect("Failed to serialize");
    
    assert!(serialized.contains("test-audience"));
    assert!(serialized.contains("test-issuer"));
    assert!(serialized.contains("aud"));
    assert!(serialized.contains("iss"));
    assert!(serialized.contains("sub"));
    assert!(serialized.contains("exp"));
    assert!(serialized.contains("jti"));
    assert!(serialized.contains("iat"));
    assert!(serialized.contains("nbf"));
}

#[test]
fn test_sau_claims_deserialization() {
    let claims = create_test_claims();
    let serialized = sonic_rs::to_string(&claims).expect("Failed to serialize");
    let deserialized: SAUClaims = sonic_rs::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(claims.aud, deserialized.aud);
    assert_eq!(claims.iss, deserialized.iss);
    assert_eq!(claims.sub, deserialized.sub);
    assert_eq!(claims.exp, deserialized.exp);
    assert_eq!(claims.jti, deserialized.jti);
    assert_eq!(claims.iat, deserialized.iat);
    assert_eq!(claims.nbf, deserialized.nbf);
}

#[test]
fn test_sau_claims_debug_format() {
    let claims = create_test_claims();
    let debug_str = format!("{:?}", claims);
    
    assert!(debug_str.contains("SAUClaims"));
    assert!(debug_str.contains("aud"));
    assert!(debug_str.contains("test-audience"));
}

#[test]
fn test_token_expiration_logic() {
    let now = Utc::now().timestamp();
    let claims = SAUClaims {
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        sub: Uuid::new_v4(),
        exp: now + 3600,
        jti: Uuid::new_v4(),
        iat: now,
        nbf: now,
    };
    
    // Token should not be expired yet
    assert!(claims.exp > Utc::now().timestamp());
    
    // Token should be valid now (nbf <= now <= exp)
    let current_time = Utc::now().timestamp();
    assert!(claims.nbf <= current_time);
    assert!(claims.exp > current_time);
}

#[test]
fn test_expired_token() {
    let past_time = Utc::now().timestamp() - 3600; // 1 hour ago
    let claims = SAUClaims {
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        sub: Uuid::new_v4(),
        exp: past_time,
        jti: Uuid::new_v4(),
        iat: past_time - 3600,
        nbf: past_time - 3600,
    };
    
    // Token should be expired
    assert!(claims.exp < Utc::now().timestamp());
}

#[test]
fn test_future_token() {
    let future_time = Utc::now().timestamp() + 3600; // 1 hour from now
    let claims = SAUClaims {
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        sub: Uuid::new_v4(),
        exp: future_time + 3600,
        jti: Uuid::new_v4(),
        iat: future_time,
        nbf: future_time,
    };
    
    // Token should not be valid yet (nbf > now)
    assert!(claims.nbf > Utc::now().timestamp());
}

#[test]
fn test_type_aliases() {
    let access_token: OAuthAccessToken = "test-access-token".to_string();
    let jwt: SAUJwt = "test-jwt-token".to_string();
    
    assert_eq!(access_token, "test-access-token");
    assert_eq!(jwt, "test-jwt-token");
}

#[test]
fn test_claims_with_different_timezones() {
    let utc_now = Utc::now();
    let timestamp = utc_now.timestamp();
    
    let claims = SAUClaims {
        aud: "test-audience".to_string(),
        iss: "test-issuer".to_string(),
        sub: Uuid::new_v4(),
        exp: timestamp + 3600,
        jti: Uuid::new_v4(),
        iat: timestamp,
        nbf: timestamp,
    };
    
    // Verify timestamps are consistent
    assert_eq!(claims.iat, timestamp);
    assert_eq!(claims.nbf, timestamp);
    assert_eq!(claims.exp, timestamp + 3600);
}