use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.omi.me/v1/dev/user/conversations";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Conversation {
    pub id: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub transcript_segments: Vec<TranscriptSegment>,
}

#[derive(Debug, Deserialize)]
pub struct TranscriptSegment {
    pub text: String,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default)]
    pub is_user: bool,
}

pub struct OmiClient {
    client: Client,
    api_key: String,
}

impl OmiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn fetch_conversations(&self) -> Result<Vec<Conversation>> {
        let response = self
            .client
            .get(BASE_URL)
            .bearer_auth(&self.api_key)
            .send()
            .context("failed to send request to Omi API")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            anyhow::bail!("Omi API returned {status}: {body}");
        }

        let conversations: Vec<Conversation> = response
            .json()
            .context("failed to deserialize Omi API response")?;

        Ok(conversations)
    }
}
