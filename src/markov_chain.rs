use std::collections::HashMap;

pub fn build_markov_chain(text: &str, k: usize) -> HashMap<&str, String> {
    let mut chain = start_building(&text, k);
    let mut state = &text[..k];
    let rest_of_text = &text[k..];

    let mut pos = 0;
    for ch in rest_of_text.chars() {
        pos += 1;
        let next_state = &text[pos..pos + k];
        chain.entry(state).and_modify(|chars| chars.push(ch));
        chain.entry(next_state).or_insert(String::new());
        state = next_state;
    }
    chain
}

fn start_building(text: &str, k: usize) -> HashMap<&str, String> {
    let mut chain = HashMap::new();
    chain.insert(&text[..k], String::new());
    chain
}
