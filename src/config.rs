use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub dns: Option<DnsConfig>,
    pub jwt: Option<JwtConfig>,
    pub secret: SecretConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DnsConfig {
    pub nameserver: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtConfig {
    pub env: Option<String>,
    pub key: Option<String>,
    pub order: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecretConfig {
    pub env: Option<String>,
    pub file: Option<String>,
    pub key: Option<String>,
    pub order: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            dns: None,
            jwt: None,
            secret: SecretConfig {
                env: Some("JIG_SECRET_KEY".to_string()),
                file: None,
                key: None,
                order: Some(vec![
                    "env".to_string(),
                    "file".to_string(),
                    "key".to_string(),
                ]),
            },
        }
    }

    pub fn from_yaml(_content: &str) -> crate::error::Result<Self> {
        // TODO: Implement proper YAML to Config deserialization with saphyr
        // For now, just return a stub config
        Ok(Config::new())
    }
}
