use sonic_rs::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}
