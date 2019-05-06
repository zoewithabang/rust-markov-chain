use indexmap::map::Entry;
use indexmap::IndexMap;
use unicase::UniCase;

pub fn build_markov_table(
    source_messages: Vec<String>,
    prefix_size: &u32,
) -> IndexMap<UniCase<String>, Vec<String>> {
    let mut table: IndexMap<UniCase<String>, Vec<String>> = IndexMap::new();
    let prefix_size: usize = *prefix_size as usize;

    for message in source_messages {
        let words = message.split_whitespace().collect::<Vec<&str>>();

        for (i, word) in words.iter().enumerate() {
            if i + prefix_size > words.len() {
                break;
            }

            let mut current_state = word.to_string();

            for j in (i + 1)..(i + prefix_size) {
                current_state = format!("{} {}", current_state, words.get(j).unwrap());
            }

            let next_state = get_next_state_string(i + prefix_size, &words);

            match table.entry(UniCase::from(current_state)) {
                Entry::Occupied(o) => o.into_mut().push(next_state),
                Entry::Vacant(v) => {
                    v.insert(vec![next_state]);
                }
            }
        }
    }

    table
}

fn get_next_state_string(next_state_index: usize, words: &Vec<&str>) -> String {
    if next_state_index < words.len() {
        words.get(next_state_index).unwrap().clone().to_string()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod markov_table_tests {
    use crate::markov_table::build_markov_table;
    use chrono::Utc;
    use unicase::UniCase;

    #[test]
    fn build_markov_table_initial_test() {
        let source_messages = vec![
            "this is a test this is the best".to_string(),
            "and this is the next best thing".to_string(),
        ];
        let prefix_size: &u32 = &2;

        let output = build_markov_table(source_messages, prefix_size);

        assert_eq!(
            output
                .get(&UniCase::from("this is".to_string()))
                .unwrap()
                .clone(),
            vec!["a".to_string(), "the".to_string(), "the".to_string()]
        );

        assert_eq!(
            output
                .get(&UniCase::from("is the".to_string()))
                .unwrap()
                .clone(),
            vec!["best".to_string(), "next".to_string()]
        );
    }

    #[test]
    fn time_for_large_message_pool() {
        let mut source_messages = vec![];

        for _ in 0..200000 {
            source_messages.push("this is a test this is the best".to_string());
        }

        let prefix_size: &u32 = &2;

        let start_time = Utc::now().timestamp_millis();

        let output = build_markov_table(source_messages, prefix_size);

        let end_time = Utc::now().timestamp_millis();

        let difference = (end_time - start_time) / 1000;

        println!("{}", difference);
    }
}
