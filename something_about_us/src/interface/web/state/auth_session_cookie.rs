use cookie::{Cookie, CookieBuilder, SameSite};
use uuid::Uuid;

use crate::infrastructure::cookie::AuthSessionCookieIssuer;
use crate::{
    domain::oauth::auth_session::AUTH_SESSION_COOKIE_NAME,
    infrastructure::config::types::SessionSecurityConfig,
};

#[derive(Clone)]
pub struct AuthSessionCookieManager {
    builder: CookieBuilder<'static>,
}

impl From<&SessionSecurityConfig> for AuthSessionCookieManager {
    fn from(value: &SessionSecurityConfig) -> Self {
        let same_site = match value.same_site.to_lowercase().as_str() {
            "strict" => SameSite::Strict,
            "lax" => SameSite::Lax,
            "none" => SameSite::None,
            _ => SameSite::Lax, // default
        };

        let builder = Cookie::build((AUTH_SESSION_COOKIE_NAME, ""))
            .http_only(value.http_only)
            .secure(value.secure_cookies)
            .same_site(same_site)
            .max_age(cookie::time::Duration::seconds(value.cookie_ttl as i64))
            .path("/");

        Self { builder }
    }
}

impl AuthSessionCookieIssuer for AuthSessionCookieManager {
    fn issuer_auth_session_cookie(&self, session_id: Uuid) -> Cookie<'static> {
        let mut builder = self.builder.clone();
        builder.inner_mut().set_value(session_id.to_string());
        builder.build()
    }
}
