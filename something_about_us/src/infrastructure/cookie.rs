use cookie::Cookie;
use uuid::Uuid;

pub trait AuthSessionCookieIssuer {
    fn issuer_auth_session_cookie(&self, session_id: Uuid) -> Cookie<'static>;
}
