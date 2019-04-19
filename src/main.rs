use chrono::prelude::*;
use dotenv;

#[derive(PartialEq)]
pub enum MarkovType {
    Users,
    Server
}

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    id: String,
    user_id: String,
    content: String,
    timestamp: NaiveDateTime
}

pub mod data;

fn main() {
    let user_ids: Vec<String> = vec!["83126901098414080".to_string(),
                                     "83132676634054656".to_string(),
                                     "83357928362344448".to_string(),
                                     "83739773033775104".to_string(),
                                     "84246523071725568".to_string(),
                                     "89718706225041408".to_string()];

    dotenv::dotenv().ok();

    let messages = data::get_stored_messages(user_ids);
}
