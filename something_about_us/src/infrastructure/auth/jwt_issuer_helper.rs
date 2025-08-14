use std::{collections::HashMap, path::PathBuf, str::FromStr};

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::jwk::{
    AlgorithmParameters, CommonParameters, EllipticCurve, Jwk, JwkSet, OctetKeyPairParameters,
    PublicKeyUse,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use crate::{
    domain::oauth::{
        error::SAUOAuthDomainError,
        sau_jwt::{SAUClaims, SAUJwt},
        sau_jwt_issuer::{JwtIssue, KeyPair, SAUJwtIssuer},
    },
    infrastructure::config::types::JwtConfig,
};

pub struct JwtIssuerHelper;

impl JwtIssuerHelper {
    pub async fn make_jwtissuer(config: &JwtConfig) -> SAUJwtIssuer {
        let helper = JwtIssuerHelper {};
        let key_pair = helper.read_or_create_key(&config).await;

        SAUJwtIssuer::new(
            config.iss.clone(),
            config.aud.clone(),
            config.access_token_ttl,
            key_pair,
        )
    }

    // if `key path + kid` exists, read the key pair from the file.
    // if not exists, generate a new key pair and write it to the file.
    // it only support Ed25519 key pair and pkcs8 format.
    async fn read_or_create_key(&self, config: &JwtConfig) -> HashMap<Uuid, KeyPair> {
        let path = PathBuf::from_str(&config.keys_path).expect("fail to parse jwt path");
        let kids = config.keys.iter().map(|k| k.kid).collect::<Vec<Uuid>>();

        let mut key_pair = HashMap::new();
        for kid in kids {
            let key = match self.read_key(path.clone(), kid).await {
                Some(key) => key,
                None => self
                    .gen_key_pair(path.clone(), kid)
                    .await
                    .expect("fail to generate jwks key pair"),
            };
            key_pair.insert(kid, key);
        }

        key_pair
    }

    async fn read_key(&self, path: PathBuf, kid: Uuid) -> Option<KeyPair> {
        let key_path_string = format!("{}/{}.pem", path.display(), kid);

        if let Ok(mut fs) = tokio::fs::File::options()
            .read(true)
            .open(key_path_string)
            .await
        {
            let mut buf = Vec::with_capacity(100);
            fs.read_to_end(&mut buf)
                .await
                .expect("fail to read jwks file");
            let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(&buf)
                .expect("fail to read pkcs8 jwk key");

            let private_key = jsonwebtoken::EncodingKey::from_ed_der(&buf);
            let public_key = jsonwebtoken::DecodingKey::from_ed_der(
                ring::signature::KeyPair::public_key(&key_pair).as_ref(),
            );

            return Some(KeyPair {
                private_key,
                public_key,
                x: BASE64_URL_SAFE_NO_PAD
                    .encode(ring::signature::KeyPair::public_key(&key_pair).as_ref()),
            });
        }
        None
    }

    async fn gen_key_pair(&self, path: PathBuf, kid: Uuid) -> Result<KeyPair, String> {
        let key_path_string = format!("{}/{}.pem", path.display(), kid);

        if let Ok(mut fs) = tokio::fs::File::options()
            .write(true)
            .create_new(true)
            .open(key_path_string.clone())
            .await
        {
            let rng = ring::rand::SystemRandom::new();
            let gen_key =
                ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).map_err(|e| e.to_string())?;
            fs.write_all(gen_key.as_ref())
                .await
                .map_err(|e| e.to_string())?;
            let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(gen_key.as_ref())
                .map_err(|e| e.to_string())?;

            return Ok(KeyPair {
                private_key: jsonwebtoken::EncodingKey::from_ed_der(gen_key.as_ref()),
                public_key: jsonwebtoken::DecodingKey::from_ed_der(
                    ring::signature::KeyPair::public_key(&key_pair).as_ref(),
                ),
                x: BASE64_URL_SAFE_NO_PAD
                    .encode(ring::signature::KeyPair::public_key(&key_pair).as_ref()),
            });
        }

        Err(format!(
            "fail to read or generate jwks pkce8 at {}",
            key_path_string
        ))
    }
}

impl JwtIssue for SAUJwtIssuer {
    fn issue_with_id(&self, kid: &Uuid, uid: &Uuid) -> Result<SAUJwt, SAUOAuthDomainError> {
        let mut header = self.header.clone();
        header.kid = Some(kid.to_string());

        let private_key = self
            .key_pair
            .get(&kid)
            .or_else(|| self.key_pair.values().next())
            .ok_or(SAUOAuthDomainError::JwtIssueFailed(format!(
                "kid : {} not found",
                kid.to_string()
            )))?
            .private_key
            .clone();

        let now = chrono::Utc::now();
        let claim = SAUClaims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            sub: uid.clone(),
            exp: (now + self.access_token_ttl.clone()).timestamp(),
            jti: Uuid::now_v7(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
        };

        let jwt = jsonwebtoken::encode(&header, &claim, &private_key)
            .map_err(|e| SAUOAuthDomainError::JwtIssueFailed(e.to_string()))?;

        Ok(jwt)
    }

    fn create_jwks(&self) -> JwkSet {
        let mut keys = Vec::new();
        for (kid, key_pair) in self.key_pair.iter() {
            let jwk = Jwk {
                common: CommonParameters {
                    key_id: Some(kid.to_string()),
                    key_algorithm: Some(jsonwebtoken::jwk::KeyAlgorithm::EdDSA),
                    public_key_use: Some(PublicKeyUse::Signature),
                    ..Default::default()
                },
                algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                    key_type: jsonwebtoken::jwk::OctetKeyPairType::OctetKeyPair,
                    curve: EllipticCurve::Ed25519,
                    x: key_pair.x.clone(),
                }),
            };
            keys.push(jwk);
        }
        JwkSet { keys }
    }
}
