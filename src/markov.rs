use indexmap::IndexMap;
use rand::prelude::*;
use std::env;
use unicase::UniCase;

pub use crate::markov_table;

pub fn generate_markov_message_with_seed(
    source_messages: Vec<String>,
    seed: Vec<String>,
) -> String {
    let prefix_size = env::var("prefix_size").unwrap().parse::<u32>().unwrap();

    println!("Got prefix size!");

    let markov_table = markov_table::build_markov_table(source_messages, &prefix_size);

    println!("Got markov table!");

    let message = generate_message_from_table_with_seed(markov_table, seed, &prefix_size);

    println!("Got message!");

    //sanitise_message(message)

    message
}

fn generate_message_from_table_with_seed(
    table: IndexMap<UniCase<String>, Vec<String>>,
    seed: Vec<String>,
    prefix_size: &u32,
) -> String {
    let max_output_size = env::var("max_output_size")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut message = get_initial_prefix(&table, seed, prefix_size);

    while message.len() <= max_output_size {
        let prefix_size = *prefix_size as usize;
        let current_prefix = &message[(message.len() - prefix_size)..];
        let current_prefix_unicase = UniCase::from(current_prefix.join(" "));

        if table.contains_key(&current_prefix_unicase) {
            message.push(get_suffix_from_suffixes(
                table.get(&current_prefix_unicase).unwrap(),
            ))
        } else {
            break;
        }
    }

    message.join(" ")
}

fn get_initial_prefix(
    table: &IndexMap<UniCase<String>, Vec<String>>,
    seed: Vec<String>,
    prefix_size: &u32,
) -> Vec<String> {
    if seed.is_empty() {
        table
            .get_index(rand::thread_rng().gen_range(0, table.len()))
            .unwrap()
            .0
            .to_string()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    } else {
        vec!["".to_string()]
    }
}

fn get_suffix_from_suffixes(suffixes: &Vec<String>) -> String {
    if suffixes.len() == 1 {
        suffixes.get(0).unwrap()
    } else {
        suffixes
            .get(rand::thread_rng().gen_range(0, suffixes.len()))
            .unwrap()
    }
    .to_string()
}

#[cfg(test)]
mod markov_tests {
    use crate::markov::get_initial_prefix;
    use indexmap::IndexMap;
    use unicase::UniCase;

    #[test]
    fn get_initial_prefix_test() {
        let mut table = IndexMap::<UniCase<String>, Vec<String>>::new();
        table.insert(
            UniCase::from("one two".to_string()),
            vec!["three".to_string()],
        );
        table.insert(
            UniCase::from("two three".to_string()),
            vec!["four".to_string()],
        );

        let seed = Vec::<String>::new();
        let prefix_size: &u32 = &2;

        let output = get_initial_prefix(&table, seed, &prefix_size);

        println!("\"{}\" == \"one two\" || \"two three\"", output.join(" "));
        assert!(output.join(" ") == "one two" || output.join(" ") == "two three");
    }
}
