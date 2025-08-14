use sonic_rs::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Token {
    pub access_token: String,
}
