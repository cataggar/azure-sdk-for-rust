use crate::TokenCredentialOptions;
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::ErrorKind,
    Error,
};

pub struct ChainedTokenCredentialCreator {
    creators: Vec<Box<dyn TokenCredentialCreator>>,
}

impl ChainedTokenCredentialCreator {
    pub fn new(creators: Vec<Box<dyn TokenCredentialCreator>>) -> Self {
        Self { creators }
    }

    pub fn create(
        &self,
        options: TokenCredentialOptions,
    ) -> azure_core::Result<ChainedTokenCredential> {
        let mut credentials: Vec<Box<dyn TokenCredential>> = Vec::new();
        for creator in &self.creators {
            match creator.create(options.clone()) {
                Ok(credential) => credentials.push(credential),
                Err(_) => continue,
            }
        }
        Ok(ChainedTokenCredential::new(credentials))
    }
}

pub trait TokenCredentialCreator {
    fn create(
        &self,
        options: TokenCredentialOptions,
    ) -> azure_core::Result<Box<dyn TokenCredential>>;
}

#[derive(Debug)]
pub struct ChainedTokenCredential {
    credentials: Vec<Box<dyn TokenCredential>>,
}

impl ChainedTokenCredential {
    pub fn new(credentials: Vec<Box<dyn TokenCredential>>) -> Self {
        Self { credentials }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ChainedTokenCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        for credential in &self.credentials {
            match credential.get_token(scopes).await {
                Ok(token) => return Ok(token),
                Err(_) => continue,
            }
        }
        Err(Error::message(
            ErrorKind::Credential,
            "no credential provided a token",
        ))
    }
    async fn clear_cache(&self) -> azure_core::Result<()> {
        for credential in &self.credentials {
            credential.clear_cache().await?;
        }
        Ok(())
    }
}
