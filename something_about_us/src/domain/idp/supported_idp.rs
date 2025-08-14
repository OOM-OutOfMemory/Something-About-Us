use crate::domain::idp::error::SupportIdpError;
use sonic_rs::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportIdp {
    Github,
}

impl SupportIdp {
    pub fn as_str(&self) -> &str {
        match self {
            SupportIdp::Github => "github",
        }
    }
}

impl TryFrom<&str> for SupportIdp {
    type Error = SupportIdpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value.to_lowercase().as_str() {
            "github" => SupportIdp::Github,
            _ => return Err(SupportIdpError::CastingError(value.to_string())),
        };
        Ok(result)
    }
}
