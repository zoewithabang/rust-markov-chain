use std::collections::HashMap;
use std::env;
use unicase::UniCase;

pub use crate::markov_table;

pub fn generate_markov_message_with_seed(source_messages: Vec<String>, seed: Vec<String>) /*-> String*/ {
    let prefix_size = env::var("prefix_size").unwrap().parse::<u32>().unwrap();

    let markov_table = markov_table::build_markov_table(source_messages, &prefix_size);

    //let message = generate_message_from_table_with_seed(markov_table, seed, &prefix_size);

    //sanitise_message(message)
}

fn generate_message_from_table_with_seed(table: HashMap<UniCase<String>, Vec<String>>, seed: Vec<String>, prefix_size: &u32) {
    let max_output_size = env::var("max_output_size").unwrap().parse::<u32>().unwrap();
}