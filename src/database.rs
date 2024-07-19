use crate::models::Ban;
use chrono::{DateTime, TimeZone, Utc};
use sqlite::{Connection, State};

pub struct Database {
    //connection: Connection,
}

impl Database {
    pub async fn new() -> Self {
        // let connection = Connection::open("broadview.db").unwrap();

        // Self { connection }
        Self {}
    }

    pub async fn find_active_ban(&self, user_id: u64) -> Option<Ban> {
        // let now = chrono::Utc::now();
        // let filter = doc! {
        //     "userId": user_id as i64,
        //     "$or": [
        //         { "expires": { "$gt": bson::DateTime::from_chrono(now)} },
        //         { "expires": { "$type": 10 } },
        //         { "expires": { "$exists": false } }
        //     ]
        // };

        // self.bans.find_one(Some(filter), None).await.unwrap()

        let connection = Connection::open("broadview.db").unwrap();
        let mut query = connection
            .prepare("SELECT * FROM bans WHERE user_id = ? AND (expires > ? OR expires IS NULL)")
            .unwrap();
        query.bind((1, user_id as i64)).unwrap();
        query.bind((2, chrono::Utc::now().timestamp())).unwrap();

        if let State::Row = query.next().unwrap() {
            let user_id = query.read::<i64, _>(0).unwrap() as u64;
            let moderator_id = query.read::<i64, _>(1).unwrap() as u64;
            let reason = query.read::<String, _>(2).unwrap();

            let expires = query.read::<String, _>(3).unwrap();
            let expires = DateTime::parse_from_rfc3339(expires.as_str())
                .unwrap()
                .to_utc();

            let timestamp = query.read::<String, _>(4).unwrap();
            let timestamp = DateTime::parse_from_rfc3339(timestamp.as_str())
                .unwrap()
                .to_utc();

            Some(Ban {
                user_id,
                moderator_id,
                reason,
                expires: Some(expires),
                timestamp,
            })
        } else {
            None
        }
    }

    pub async fn insert_ban(&self, ban: Ban) -> Result<(), String> {
        let connection = Connection::open("broadview.db").unwrap();
        let mut query = connection
			.prepare("INSERT INTO bans (user_id, moderator_id, reason, expires, timestamp) VALUES (?, ?, ?, ?, ?)")
			.unwrap();
        query.bind(1, ban.user_id as i64).unwrap();
        query.bind(2, ban.moderator_id as i64).unwrap();
        query.bind(3, ban.reason.as_str()).unwrap();
        query
            .bind(
                4,
                ban.expires
                    .map(|expires| expires.to_rfc3339().as_str())
                    .unwrap_or(None),
            )
            .unwrap();
        query.bind(5, ban.timestamp.to_rfc3339().as_str()).unwrap();

        query.next().unwrap();

        Ok(())
    }

    pub async fn remove_ban(&self, user_id: u64) -> Result<(), String> {
        todo!();
        // if self.find_active_ban(user_id).await.is_none() {
        //     Err("User is not banned".to_string())
        // } else {
        //     let filter = doc! { "userId": user_id as i64 };
        //     self.bans.delete_one(filter, None).await.unwrap();
        //     Ok(())
        // }
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
