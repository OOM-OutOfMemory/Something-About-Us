use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct JwkSetDoc {
    /// Array of JWK public keys.
    pub keys: Vec<JwkDoc>,
}

// JSON Web Key (JWK) document model for documentation purposes.
// represents Ed25519 OKP public keys exposed by this service.
#[derive(ToSchema)]
pub struct JwkDoc {
    #[schema(example = "OKP")]
    pub kty: String,

    #[schema(example = "Ed25519")]
    pub crv: Option<String>,

    // the public key, base64url encoded.
    #[schema(example = "VGhpc0lzTm90QVNlY3JldEtleQ")]
    pub x: Option<String>,

    // intended use for the key. Commonly "sig" (signature).
    #[schema(example = "sig")]
    pub r#use: Option<String>,

    #[schema(example = "EdDSA")]
    pub alg: Option<String>,

    #[schema(example = "00000000-0000-7000-8000-000000000000")]
    pub kid: Option<String>,
}
