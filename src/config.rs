use crate::models::Issuer;
use lazy_static::lazy_static;
use p12_keystore::KeyStore;
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey};
use std::sync::RwLock;
use thiserror::Error;

#[derive(Clone)]
pub struct Certificate {
    pub private_key: RsaPrivateKey,
    pub public_key: p12_keystore::Certificate,
}

impl Certificate {
    pub fn load(config: &PKCS12Config) -> Result<Self, ConfigError> {
        let (data, password) = match &config {
            PKCS12Config::File { path, password } => {
                let data = std::fs::read(path).map_err(ConfigError::CertificateReadFileError)?;
                (data, password.as_str())
            }
            PKCS12Config::InMemory { data, password } => (data.to_vec(), password.as_str()),
        };

        let keystore = KeyStore::from_pkcs12(&data, password)
            .map_err(ConfigError::CertificateKeyStoreError)?;

        let private_entry = keystore
            .private_key_chain()
            .ok_or(ConfigError::CertificateMissingPrivateKeyError)?;

        let private_key = RsaPrivateKey::from_pkcs8_der(private_entry.1.key())
            .map_err(ConfigError::CertificateParsingPrivateKeyError)?;

        let first_certificate = private_entry
            .1
            .chain()
            .first()
            .ok_or(ConfigError::CertificateMissingPublicKeyError)?;

        Ok(Certificate {
            private_key,
            public_key: first_certificate.clone(),
        })
    }
}

struct InnerConfig {
    pub issuer: Issuer,
    pub certificate: Certificate,
}

impl TryFrom<Config> for InnerConfig {
    type Error = ConfigError;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let certificate = Certificate::load(&config.pkcs12_config)?;
        Ok(InnerConfig {
            issuer: config.issuer,
            certificate,
        })
    }
}

pub enum PKCS12Config {
    File { path: String, password: String },
    InMemory { data: Vec<u8>, password: String },
}

impl PKCS12Config {
    pub fn new_from_file(path: String, password: String) -> Self {
        PKCS12Config::File { path, password }
    }

    pub fn new_in_memory(data: Vec<u8>, password: String) -> Self {
        PKCS12Config::InMemory { data, password }
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

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid issuer configuration")]
    InvalidIssuer,
    #[error("Failed to load certificate: {0}")]
    CertificateReadFileError(#[from] std::io::Error),
    #[error("Failed to load certificate: {0}")]
    CertificateKeyStoreError(#[from] p12_keystore::error::Error),
    #[error("Failed to parse private key: {0}")]
    CertificateParsingPrivateKeyError(#[from] rsa::pkcs8::Error),
    #[error("Certificate is missing a private key")]
    CertificateMissingPrivateKeyError,
    #[error("Certificate is missing a public key")]
    CertificateMissingPublicKeyError,
    #[error("Failed to parse public key: {0}")]
    CertificateRsaParsingPublicKeyError(#[from] rsa::pkcs8::spki::Error),
    #[error("Configuration is locked")]
    Locked,
    #[error("Configuration is not initialized")]
    NotInitialized,
}

lazy_static! {
    static ref CONFIG: RwLock<Option<InnerConfig>> = RwLock::new(None);
}

pub fn set_config(config: Config) -> Result<(), ConfigError> {
    let mut config_lock = CONFIG.write().map_err(|_| ConfigError::Locked)?;
    let inner_config = InnerConfig::try_from(config)?;
    *config_lock = Some(inner_config);
    Ok(())
}

pub fn clear_config() -> Result<(), ConfigError> {
    let mut config_lock = CONFIG.write().map_err(|_| ConfigError::Locked)?;
    *config_lock = None;
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
    let config_lock = CONFIG.read().expect("CONFIG lock is poisoned");
    config_lock.is_some()
}

pub fn certificate() -> Result<Certificate, ConfigError> {
    let config_lock = CONFIG.read().map_err(|_| ConfigError::Locked)?;
    if let Some(ref config) = *config_lock {
        Ok(config.certificate.clone())
    } else {
        Err(ConfigError::NotInitialized)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::tests::setup_issuer;
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rusty_fork::rusty_fork_test;

    const TEST_CERTIFICATE_PATH: &str = "./tests/credentials/cert.p12";
    const TEST_CERTIFICATE_DATA: &[u8] = include_bytes!("../tests/credentials/cert.p12");
    const TEST_CERTIFICATE_PASSWORD: &str = "12345678";
    const TEST_CERTIFICATE_PRIVATE_KEY: &str = include_str!("../tests/credentials/key.pem");
    const TEST_CERTIFICATE_PUBLIC_KEY: &str = include_str!("../tests/credentials/key.crt");

    pub fn setup_test_config() -> Config {
        Config::new(setup_issuer(), setup_pkcs12_in_memory_config())
    }

    fn setup_pkcs12_in_memory_config() -> PKCS12Config {
        PKCS12Config::new_in_memory(
            TEST_CERTIFICATE_DATA.to_vec(),
            TEST_CERTIFICATE_PASSWORD.to_string(),
        )
    }

    fn setup_pkcs12_from_file_config() -> PKCS12Config {
        PKCS12Config::new_from_file(
            TEST_CERTIFICATE_PATH.to_string(),
            TEST_CERTIFICATE_PASSWORD.to_string(),
        )
    }

    fn test_config(issuer: Issuer, pkcs12_config: PKCS12Config) {
        let config = Config::new(issuer.clone(), pkcs12_config);

        assert!(!is_set());
        set_config(config).expect("Failed to set config");
        assert!(is_set());

        let retrieved_issuer = get_issuer().expect("Failed to retrieve issuer");
        assert_eq!(retrieved_issuer, issuer);

        clear_config().expect("Failed to clear config");
        assert!(!is_set());
    }

    #[test]
    fn test_load_certificate_from_file() {
        let cert = Certificate::load(&setup_pkcs12_from_file_config())
            .expect("Failed to load certificate from file");

        let private_key_pem = cert
            .private_key
            .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
            .expect("Failed to parse private key from certificate");
        assert_eq!(private_key_pem.as_str(), TEST_CERTIFICATE_PRIVATE_KEY);

        use base64::prelude::*;
        let public_key_der = cert.public_key.as_der();
        assert_eq!(
            BASE64_STANDARD.encode(public_key_der),
            TEST_CERTIFICATE_PUBLIC_KEY[..TEST_CERTIFICATE_PUBLIC_KEY.len() - 1]
        );
    }

    rusty_fork_test! {
        #[test]
        fn test_set_and_get_config() {
            let issuer = setup_issuer();
            let pkcs12_config = PKCS12Config::new_in_memory(
                TEST_CERTIFICATE_DATA.to_vec(),
                TEST_CERTIFICATE_PASSWORD.to_string(),
            );
            test_config(issuer, pkcs12_config);
        }
    }

    rusty_fork_test! {
        #[test]
        fn test_set_and_get_config_from_file() {
            let issuer = setup_issuer();
            let pkcs12_config = PKCS12Config::new_from_file(
                TEST_CERTIFICATE_PATH.to_string(),
                TEST_CERTIFICATE_PASSWORD.to_string(),
            );
            test_config(issuer, pkcs12_config);
        }
    }
}
