use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub fn write(chain: HashMap<&str, String>, l: usize) {
    let mut state = start_writing(&chain);
    print!("{}", state);
    let mut count = state.chars().count();

    let mut rng = thread_rng();
    while count < l {
        let chars = chain.get(state.as_str()).expect("Something went wrong!");
        let chosen = chars.chars().choose(&mut rng);
        match chosen {
            None => {
                // Restart writing
                state = start_writing(&chain);
            }
            Some(ch) => {
                print!("{}", ch);
                state.remove(0);
                state.push(ch);
                count += 1;
            }
        }
    }
}

fn start_writing(chain: &HashMap<&str, String>) -> String {
    let mut rng = thread_rng();

    let state = chain.keys().choose(&mut rng).expect("The chain is empty!");
    return String::from(*state);
}
