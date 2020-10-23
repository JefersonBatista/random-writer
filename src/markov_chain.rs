use std::collections::HashMap;

pub fn build(text: &str, k: usize) -> HashMap<&str, String> {
    let mut chain = start_building(&text, k);
    let first_state_byte_count = after_last_char_pos(&text, 0, k);
    let mut state = &text[..first_state_byte_count];
    let rest_of_text = &text[first_state_byte_count..];

    let mut begin = 0;
    let mut end = first_state_byte_count;

    for ch in rest_of_text.chars() {
        begin = after_last_char_pos(&text, begin, 1);
        end = after_last_char_pos(&text, end, 1);

        let next_state = &text[begin..end];
        chain.entry(state).and_modify(|edges| edges.push(ch));
        chain.entry(next_state).or_insert(String::new());
        state = next_state;
    }
    chain
}

fn start_building(text: &str, k: usize) -> HashMap<&str, String> {
    let mut chain = HashMap::new();
    chain.insert(&text[..after_last_char_pos(&text, 0, k)], String::new());
    chain
}

/**
 * Rust slices str by bytes, not by chars.
 * To deal with it, this function receives a text (&str),
 * a initial position (for bytes) and the quantity of
 * desired chars, and returns the position of the next byte
 * after the last byte of the last char. The init must be
 * char boundary for text.
 */
pub fn after_last_char_pos(text: &str, init: usize, qtd: usize) -> usize {
    if qtd < 1 {
        init
    } else {
        let mut pos = init;

        for _ in 0..qtd {
            pos += 1;
            while !text.is_char_boundary(pos) {
                pos += 1;
            }
        }

        pos
    }
}
