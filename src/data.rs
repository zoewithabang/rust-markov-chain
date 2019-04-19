use mysql;
use rand::prelude::*;
use std::env;

struct MessageSearchOptions {
    message_count: u32,
    message_count_offset: u32
}

pub fn get_stored_messages(user_ids: Vec<String>) /*-> Vec<String>*/ {
    const MARKOV_TYPE: crate::MarkovType = crate::MarkovType::Users;
    const MAX_LENGTH: u32 = 15;
    const MAX_MESSAGE_COUNT: u32 = 20;

    let pool = mysql::Pool::new(get_database_string()).unwrap();

    let messages = match MARKOV_TYPE {
        crate::MarkovType::Users => {
            let user_message_count = get_users_message_count(&pool, &user_ids);
            let options = get_message_search_options(&user_message_count, &MAX_MESSAGE_COUNT);

            get_messages_for_users(&pool, &user_ids, options)
        },
        crate::MarkovType::Server => {
            let server_message_count = get_server_message_count(&pool);
            let options = get_message_search_options(&server_message_count, &MAX_MESSAGE_COUNT);

            get_messages_for_users(&pool, &user_ids, options)
        }
    };

    for message in messages {
        println!("{:?}", message);
    }
}

fn get_users_message_count(pool: &mysql::Pool, user_ids: &Vec<String>) -> u32 {
    let mut query = String::from("SELECT COUNT(*) AS total FROM messages WHERE");
    let mut params: Vec<mysql::Value> = Vec::with_capacity(user_ids.len());

    for (i, user_id) in user_ids.iter().enumerate() {
        query.push_str(" user_id = ?");

        if i != user_ids.len() - 1 {
            query.push_str(" OR");
        }

        params.push(user_id.into())
    }

    pool.prep_exec(query, params)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|row| {
            mysql::from_row::<u32>(row)
        })
        .collect::<Vec<u32>>()
        .first()
        .cloned()
        .unwrap()
}

fn get_server_message_count(pool: &mysql::Pool) -> u32 {
    let query = String::from("SELECT COUNT(*) AS total FROM messages");

    pool.prep_exec(query, ())
        .unwrap()
        .map(|x| x.unwrap())
        .map(|row| {
            mysql::from_row::<u32>(row)
        })
        .collect::<Vec<u32>>()
        .first()
        .cloned()
        .unwrap()
}

fn get_message_search_options(user_message_count: &u32, max_count: &u32) -> MessageSearchOptions {
    if user_message_count < max_count {
        let message_count = user_message_count.clone();
        let message_count_offset = 0;

        return MessageSearchOptions { message_count, message_count_offset }
    }
    else {
        let message_count = max_count.clone();
        let message_count_offset = rand::thread_rng().gen_range(0, user_message_count - message_count);

        return MessageSearchOptions { message_count, message_count_offset }
    }
}

fn get_messages_for_users(pool: &mysql::Pool, user_ids: &Vec<String>, options: MessageSearchOptions) -> Vec<String> {
    let mut query = String::from("SELECT * FROM messages WHERE");
    let mut params: Vec<mysql::Value> = Vec::with_capacity(user_ids.len() + 2);

    for (i, user_id) in user_ids.iter().enumerate() {
        query.push_str(" user_id = ?");

        if i != user_ids.len() - 1 {
            query.push_str(" OR");
        }

        params.push(user_id.into())
    }

    query.push_str(" ORDER BY RAND() LIMIT ?,?");
    params.push(options.message_count_offset.into());
    params.push(options.message_count.into());

    pool.prep_exec(query, params)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|row| {
            let (id, user_id, content, timestamp) = mysql::from_row(row);
            crate::Message {
                id,
                user_id,
                content,
                timestamp
            }
        })
        .collect::<Vec<crate::Message>>()
        .iter()
        .map(|x| x.content.clone())
        .collect::<Vec<String>>()
}

fn get_database_string() -> String {
    format!("mysql://{}:{}@{}:{}/{}",
            env::var("dbuser").unwrap(),
            env::var("dbpassword").unwrap(),
            env::var("dbaddress").unwrap(),
            env::var("dbport").unwrap(),
            env::var("dbdatabase").unwrap())
}
