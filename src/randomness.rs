use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub fn test_choose(state: &str, qtd: u32) {
    let mut rng = thread_rng();

    let mut count_letters = HashMap::new();

    for _ in 0..qtd {
        let chosen = match state.chars().choose(&mut rng) {
            Some(letter) => letter,
            None => continue,
        };

        let count = count_letters.entry(chosen).or_insert(0);
        *count += 1;
    }

    for (letter, count) in &count_letters {
        println!("{}: {}", letter, count);
    }
}
