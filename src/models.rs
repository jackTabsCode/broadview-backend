use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub user_id: u64,
    pub moderator_id: u64,

    pub reason: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "opt_bson_datetime_as_rfc3339_string")]
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

mod opt_bson_datetime_as_rfc3339_string {
    use bson::DateTime;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => date.try_to_rfc3339_string().unwrap().serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date = String::deserialize(deserializer)?;
        if date.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                DateTime::parse_rfc3339_str(&date).map_err(serde::de::Error::custom)?,
            ))
        }
    }
}
