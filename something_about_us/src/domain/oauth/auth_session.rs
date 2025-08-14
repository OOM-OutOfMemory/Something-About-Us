use sonic_rs::{Deserialize, Serialize};
use uuid::Uuid;

pub const AUTH_SESSION_COOKIE_NAME: &str = "something_about_us_auth_session";
pub const AUTH_HTTP_AGENT_NAME: &str = "SomethingAboutUs";

#[derive(Serialize, Deserialize)]
pub struct AuthSession {
    pub id: Uuid,
    pub pkce_verifier: String,
    pub csrf_token: String,
}
