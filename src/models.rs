use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub user_id: u64,
    pub moderator_id: u64,

    pub reason: String,

    pub expires: Option<DateTime<Utc>>,
    pub timestamp: DateTime<Utc>,
}

impl Ban {
    pub fn from_request(request: V1BanRequest, user_id: u64) -> Self {
        Self {
            user_id,
            moderator_id: request.moderator_id,
            reason: request.reason,
            expires: request.expires,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BanRequest {
    pub moderator_id: u64,
    pub reason: String,
    pub expires: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Ban {
    pub user_id: u64,
    pub moderator_id: u64,
    pub reason: String,
    pub expires: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum V1BanResponse {
    NotBanned {
        banned: serde_bool::False,
    },
    Banned {
        banned: serde_bool::True,

        #[serde(flatten)]
        ban: V1Ban,
    },
}

impl From<Ban> for V1Ban {
    fn from(document: Ban) -> Self {
        Self {
            user_id: document.user_id,
            moderator_id: document.moderator_id,
            reason: document.reason,
            expires: document.expires.map(|expires| expires.to_rfc3339()),
            timestamp: document.timestamp.to_rfc3339(),
        }
    }
}
