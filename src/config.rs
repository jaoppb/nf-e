use crate::models::Issuer;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct PKCS12Config {
    pub path: String,
    pub password: String,
}

impl PKCS12Config {
    pub fn new(path: String, password: String) -> Self {
        PKCS12Config { path, password }
    }
}

pub struct Config {
    issuer: Issuer,
    pkcs12_config: PKCS12Config,
}

impl Config {
    pub fn new(issuer: Issuer, pkcs12_config: PKCS12Config) -> Self {
        Config {
            issuer,
            pkcs12_config,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    InvalidIssuer,
    MissingPKCS12Config,
    Locked,
    NotInitialized,
}

lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::new(None);
}

pub fn set_config(config: Config) -> Result<(), ConfigError> {
    let mut config_lock = CONFIG.write().map_err(|_| ConfigError::Locked)?;
    *config_lock = Some(config);
    Ok(())
}

pub fn get_issuer() -> Result<Issuer, ConfigError> {
    let config_lock = CONFIG.read().map_err(|_| ConfigError::Locked)?;
    if let Some(ref config) = *config_lock {
        Ok(config.issuer.clone())
    } else {
        Err(ConfigError::NotInitialized)
    }
}

pub fn is_set() -> bool {
    let config_lock = CONFIG
        .read()
        .expect("CONFIG lock is poisoned");
    config_lock.is_some()
}

pub fn get_pkcs12_certificate() -> Result<(), ConfigError> {
    todo!("Implement PKCS#12 certificate loading logic here");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tests::setup_issuer;

    #[test]
    fn test_set_and_get_config() {
        let issuer = setup_issuer();
        let pkcs12_config =
            PKCS12Config::new("path/to/cert.p12".to_string(), "password".to_string());
        let config = Config::new(issuer.clone(), pkcs12_config);

        assert!(!is_set());
        set_config(config).unwrap();
        assert!(is_set());

        let retrieved_issuer = get_issuer().unwrap();
        assert_eq!(retrieved_issuer, issuer);
    }
}
