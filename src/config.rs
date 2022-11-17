use std::collections::HashMap;

use crate::clickup::{self, team::TeamId, webhooks::request::CreateWebhookParameters};

pub struct Config {
    credentials: Vec<Credential>,
    webhooks: Vec<Webhook>,
}

pub enum Credential {
    Authorization { name: String, token: String },
}

#[derive(Hash)]
pub enum Webhook {
    ClickUp {
        team_id: TeamId,
        request: CreateWebhookParameters,
        credentials: String,
    },
}

impl Webhook {
    async fn create(&self, cred: &Credential) -> color_eyre::eyre::Result<String> {
        match self {
            Self::ClickUp {
                team_id, request, ..
            } => {
                let token = match cred {
                    Credential::Authorization { name, token } => token,
                };
                clickup::webhooks::request::create_webhook(*team_id, request.clone(), token.clone())
                    .await
                    .map_err(|e| e.into())
                    .into()
            }
        }
    }
}
