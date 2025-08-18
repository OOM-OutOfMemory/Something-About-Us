use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, IntoParams, ToSchema)]
#[into_params(parameter_in = Path)]
pub struct IdpPathParam {
    #[param(value_type = String, example = "github")]
    pub idp: String,
}
