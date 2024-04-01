use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanDocument {
    pub user_id: u64,
    pub moderator_id: u64,

    pub reason: String,

    pub expires: Option<bson::DateTime>,
    pub timestamp: bson::DateTime,
}

impl BanDocument {
    pub fn from_request(request: V1BanRequest, user_id: u64) -> Self {
        Self {
            user_id,
            moderator_id: request.moderator_id,
            reason: request.reason,
            expires: request.expires.map(bson::DateTime::from_chrono),
            timestamp: bson::DateTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1BanRequest {
    pub moderator_id: u64,
    pub reason: String,
    pub expires: Option<chrono::DateTime<Utc>>,
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

impl From<BanDocument> for V1Ban {
    fn from(document: BanDocument) -> Self {
        Self {
            user_id: document.user_id,
            moderator_id: document.moderator_id,
            reason: document.reason,
            expires: document
                .expires
                .map(|date| date.try_to_rfc3339_string().unwrap()),
            timestamp: document.timestamp.try_to_rfc3339_string().unwrap(),
        }
    }
}
