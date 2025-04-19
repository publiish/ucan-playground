use super::types::UcanToken;
use anyhow::{bail, Result};
use async_trait::async_trait;
use base64::prelude::*;
use chrono::{Duration, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use gloo_timers::future::TimeoutFuture;
use rand::rngs::OsRng;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing;
use ucan::{
    builder::UcanBuilder,
    capability::proof::{ProofAction, ProofDelegationSemantics, ProofSelection},
    chain::{CapabilityInfo, ProofChain},
    crypto::{did::DidParser, KeyMaterial},
    store::MemoryStore,
};
use web_sys;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Ed25519Key(SigningKey);

impl Ed25519Key {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        Ed25519Key(SigningKey::generate(&mut csprng))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.0.verifying_key()
    }
}

#[async_trait(?Send)]
impl KeyMaterial for Ed25519Key {
    fn get_jwt_algorithm_name(&self) -> String {
        "EdDSA".to_string()
    }

    async fn get_did(&self) -> Result<String> {
        Ok(format!(
            "did:key:{}",
            BASE64_STANDARD.encode(self.to_bytes())
        ))
    }

    async fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(payload).to_bytes().to_vec())
    }

    async fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()> {
        let sig = match <[u8; 64]>::try_from(signature) {
            Ok(bytes) => Signature::from_bytes(&bytes),
            Err(e) => return Err(anyhow::anyhow!("Invalid signature length: {:?}", e)),
        };

        self.verifying_key()
            .verify(payload, &sig)
            .map_err(|e| anyhow::anyhow!("Verification failed: {}", e))
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct UcanPlayground {
    pub tokens: Vec<UcanToken>,
    keypair: Ed25519Key,
}

impl UcanPlayground {
    pub fn new() -> Self {
        UcanPlayground {
            tokens: Vec::new(),
            keypair: Ed25519Key::new(),
        }
    }

    pub async fn create_root_ucan(
        &mut self,
        audience_did: &str,
        scope: String,
    ) -> Result<UcanToken> {
        let did_regex = Regex::new(r"^did:key:[a-zA-Z0-9+\/=]+$").unwrap();
        if !did_regex.is_match(audience_did) {
            bail!("Invalid audience DID format");
        }
        if scope.is_empty() {
            bail!("Scope cannot be empty");
        }

        let issuer_did = format!(
            "did:key:{}",
            BASE64_STANDARD.encode(self.keypair.to_bytes())
        );

        let token = UcanBuilder::default()
            .issued_by(&self.keypair)
            .for_audience(audience_did)
            .with_lifetime(3600)
            .claiming_capability(ucan::capability::Capability {
                resource: scope.clone(),
                ability: ProofAction::Delegate.to_string(),
                caveat: json!({}),
            })
            .build()?
            .sign()
            .await?;

        let jwt = token.encode()?;
        let ucan_token = UcanToken {
            jwt,
            issuer: issuer_did.clone(),
            audience: audience_did.to_string(),
            expiration: Utc::now() + Duration::seconds(3600),
        };

        self.tokens.push(ucan_token.clone());
        tracing::info!(
            issuer = %issuer_did,
            audience = %audience_did,
            scope = %scope,
            "Created root UCAN"
        );
        Ok(ucan_token)
    }

    pub async fn delegate_ucan(
        &mut self,
        parent_jwt: &str,
        audience_did: &str,
        scope: String,
    ) -> Result<UcanToken> {
        let did_regex = Regex::new(r"^did:key:[a-zA-Z0-9+\/=]+$").unwrap();
        if !did_regex.is_match(audience_did) {
            bail!("Invalid audience DID format");
        }
        if scope.is_empty() {
            bail!("Scope cannot be empty");
        }

        let issuer_did = format!(
            "did:key:{}",
            BASE64_STANDARD.encode(self.keypair.to_bytes())
        );

        let mut did_parser = DidParser::new(&[]);
        let store = MemoryStore::default();
        let parent_chain =
            ProofChain::try_from_token_string(parent_jwt, None, &mut did_parser, &store).await?;
        let parent_ucan = parent_chain.ucan();

        let token = UcanBuilder::default()
            .issued_by(&self.keypair)
            .for_audience(audience_did)
            .with_lifetime(1800)
            .claiming_capability(ucan::capability::Capability {
                resource: scope.clone(),
                ability: ProofAction::Delegate.to_string(),
                caveat: json!({}),
            })
            .witnessed_by(parent_ucan, None)
            .build()?
            .sign()
            .await?;

        let jwt = token.encode()?;
        let ucan_token = UcanToken {
            jwt,
            issuer: issuer_did.clone(),
            audience: audience_did.to_string(),
            expiration: Utc::now() + Duration::seconds(1800),
        };

        self.tokens.push(ucan_token.clone());
        tracing::info!(
            issuer = %issuer_did,
            audience = %audience_did,
            scope = %scope,
            parent_jwt = %parent_jwt,
            "Created delegated UCAN"
        );
        Ok(ucan_token)
    }

    pub async fn verify_ucan(
        &self,
        jwt: &str,
    ) -> Result<Vec<CapabilityInfo<ProofSelection, ProofAction>>> {
        let mut did_parser = DidParser::new(&[]);
        let store = MemoryStore::default();
        let proof_chain =
            ProofChain::try_from_token_string(jwt, None, &mut did_parser, &store).await?;
        Ok(proof_chain.reduce_capabilities(&ProofDelegationSemantics {}))
    }

    pub async fn publish_to_node(&self, jwt: &str) -> Result<String> {
        let max_retries = 3;
        let mut attempts = 0;
        let client = Client::new();

        loop {
            let response = client
                .post("http://localhost:3000/publiish")
                .header("Authorization", format!("Bearer {}", jwt))
                .send()
                .await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    let result = resp.text().await?;
                    tracing::info!(jwt = %jwt, "Published to node successfully");
                    return Ok(result);
                }
                Ok(resp) => {
                    bail!("Failed to publish: {}", resp.status());
                }
                Err(e) if attempts < max_retries => {
                    attempts += 1;
                    tracing::warn!(
                        attempt = attempts,
                        max_retries = max_retries,
                        error = %e,
                        "Retry publishing to node"
                    );
                    TimeoutFuture::new((100 * 2u32.pow(attempts as u32)) as u32).await;
                    continue;
                }
                Err(e) => bail!("Failed to publish after {} attempts: {}", max_retries, e),
            }
        }
    }

    pub fn get_tokens(&self) -> &Vec<UcanToken> {
        &self.tokens
    }

    pub fn save_to_storage(&self) -> Result<()> {
        let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("No window object"))?;
        let storage = window
            .local_storage()
            .map_err(|e| anyhow::anyhow!("Local storage error: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("No localStorage"))?;
        let tokens_json = serde_json::to_string(&self.tokens)?;
        storage
            .set_item("ucan_tokens", &tokens_json)
            .map_err(|e| anyhow::anyhow!("Storage error: {:?}", e))?;
        tracing::info!(
            tokens_count = self.tokens.len(),
            "Saved tokens to localStorage"
        );
        Ok(())
    }

    pub fn load_from_storage(&mut self) -> Result<()> {
        let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("No window object"))?;
        let storage = window
            .local_storage()
            .map_err(|e| anyhow::anyhow!("Local storage error: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("No localStorage"))?;
        if let Some(tokens_json) = storage
            .get_item("ucan_tokens")
            .map_err(|e| anyhow::anyhow!("Storage error: {:?}", e))?
        {
            self.tokens = serde_json::from_str(&tokens_json)?;
            tracing::info!(
                tokens_count = self.tokens.len(),
                "Loaded tokens from localStorage"
            );
        }
        Ok(())
    }
}
