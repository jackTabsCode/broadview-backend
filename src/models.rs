use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub user_id: u64,
    pub moderator_id: u64,

    pub reason: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<DateTime>,

    pub timestamp: DateTime,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum V1BanResult {
    NotBanned {
        banned: serde_bool::False,
    },
    Banned {
        banned: serde_bool::True,

        #[serde(flatten)]
        ban: Ban,
    },
}
