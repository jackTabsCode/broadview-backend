// use crate::guards::api_key::ApiKey;
// use roboat::Client;
// use rocket::State;

// const GROUP_ID: u64 = 3016035;
// const ROLE_ID: u64 = 20676927;

// #[post("/resident/<user_id>")]
// pub async fn make_resident(_api_key: ApiKey, roboat_client: &State<Client>, user_id: &str) {
//     let user_id = user_id.parse::<u64>().unwrap();

//     roboat_client
//         .set_group_member_role(user_id, GROUP_ID, ROLE_ID)
//         .await
//         .unwrap();
// }
