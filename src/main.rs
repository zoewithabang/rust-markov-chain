extern crate dotenv;
#[macro_use]
extern crate mysql;

use dotenv::dotenv;
use std::char;
use std::env;

#[derive(PartialEq)]
enum MarkovType {
    Single,
    Mashup,
    Server
}

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: String,
    tracked: u8,
    permission_rank: u8
}

fn main() {
    const MESSAGE_COUNT: u32 = 200_000;
    const MAX_LENGTH: u32 = 15;
    const MARKOV_TYPE: MarkovType = MarkovType::Server;

    let user_ids: Vec<String> = vec!["83126901098414080".to_string(),
                                     "83132676634054656".to_string(),
                                     "83357928362344448".to_string(),
                                     "83739773033775104".to_string(),
                                     "84246523071725568".to_string(),
                                     "89718706225041408".to_string()];

    dotenv().ok();

    let messages = get_stored_messages(user_ids, MESSAGE_COUNT, MARKOV_TYPE);
}

fn get_stored_messages(user_ids: Vec<String>, message_count: u32, markov_type: MarkovType) /*-> Vec<String>*/ {
    match markov_type {
        MarkovType::Single => println!("Single"),
        MarkovType::Mashup => println!("Mashup"),
        MarkovType::Server => println!("Server"),
    };

    let db = get_database_string();

    println!("{}", db);

    let pool = mysql::Pool::new(db).unwrap();

    let mut query = String::from("SELECT * FROM users WHERE");
    let mut params: Vec<mysql::Value> = Vec::with_capacity(user_ids.len());

    for (i, user_id) in user_ids.iter().enumerate() {
        query.push_str(" id = :id");
        query.push(char::from_digit(i as u32, 10).unwrap());

        if i != user_ids.len() - 1 {
            query.push_str(" OR");
        }

        params.push(user_id.into())
    }

    let users = pool.prepare(query)
        .unwrap()
        .execute(params)
        .unwrap()
        .map(|x| x.unwrap())
            .map(|row| {
                let (id, tracked, permission_rank) = mysql::from_row(row);
                User {
                    id,
                    tracked,
                    permission_rank
                }
            })
        .collect::<Vec<User>>();

    for user in users {
        println!("{:?}", user);
    }
}

fn get_database_string() -> String {
    format!("mysql://{}:{}@{}:{}/{}",
            env::var("dbuser").unwrap(),
            env::var("dbpassword").unwrap(),
            env::var("dbaddress").unwrap(),
            env::var("dbport").unwrap(),
            env::var("dbdatabase").unwrap())
}
