use mongodb::{bson::doc, options::ClientOptions, Collection};

use crate::models::BanDocument;

pub struct Database {
    bans: Collection<BanDocument>,
}

impl Database {
    pub async fn new() -> Self {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        client_options.app_name = Some("Broadview Backend".to_string());

        let client = mongodb::Client::with_options(client_options).unwrap();

        let db = client.database("broadview");
        let col: Collection<BanDocument> = db.collection("bans");

        Self { bans: col }
    }

    pub async fn find_active_ban(&self, user_id: u64) -> Option<BanDocument> {
        let now = chrono::Utc::now();
        let filter = doc! {
            "userId": user_id as i64,
            "$or": [
                { "expires": { "$gt": bson::DateTime::from_chrono(now)} },
                { "expires": { "$type": 10 } },
                { "expires": { "$exists": false } }
            ]
        };

        self.bans.find_one(Some(filter), None).await.unwrap()
    }

    pub async fn insert_ban(&self, ban: BanDocument) -> Result<(), String> {
        if self.find_active_ban(ban.user_id).await.is_some() {
            Err("User is already banned".to_string())
        } else {
            self.bans.insert_one(ban, None).await.unwrap();
            Ok(())
        }
    }

    pub async fn remove_ban(&self, user_id: u64) -> Result<(), String> {
        if self.find_active_ban(user_id).await.is_none() {
            Err("User is not banned".to_string())
        } else {
            let filter = doc! { "userId": user_id as i64 };
            self.bans.delete_one(filter, None).await.unwrap();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_active_ban() {
        let database = Database::new().await;
        let test = database.find_active_ban(1475049546).await;

        assert!(test.is_some());
    }
}
