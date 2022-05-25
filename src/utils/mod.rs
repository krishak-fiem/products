use models::products::user::User;
use serde_json::from_str;

pub async fn create_user_from_kafka(json_string: &str) {
    let user: User = from_str(json_string).unwrap();
    user.add_user().await;
}
