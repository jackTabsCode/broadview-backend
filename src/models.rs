use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub user_id: u64,
    pub moderator_id: u64,

    pub reason: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<DateTime>,

    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
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
