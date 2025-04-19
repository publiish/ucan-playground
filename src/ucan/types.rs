use serde::{Deserialize, Serialize};
use ucan::capability::Scope;
use url::Url;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct PublishScope(pub String);

impl Scope for PublishScope {
    fn contains(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for PublishScope {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<Url> for PublishScope {
    type Error = anyhow::Error;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Ok(PublishScope(url.to_string()))
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct UcanToken {
    pub jwt: String,
    pub issuer: String,
    pub audience: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
}
