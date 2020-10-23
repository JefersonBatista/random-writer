use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

pub fn write(chain: HashMap<&str, String>, l: usize) {
    let mut state = start_writing(&chain);
    print!("{}", state);
    let k = state.len();

    let mut count = k;
    let mut rng = thread_rng();
    loop {
        let chars = chain.get(&state[..]).expect("Something went wrong!");
        let chosen = chars.chars().choose(&mut rng);
        match chosen {
            None => {
                // Restart writing
                state = start_writing(&chain);
                // print!("{}", state);
                // count += k;
            },
            Some(ch) => {
                print!("{}", ch);
                state = String::from(&state[1..]);
                state.push(ch);
                count += 1;
            }
        }
        
        if count >= l {
            break;
        }
    }
}

fn start_writing(chain: &HashMap<&str, String>) -> String {
    let mut rng = thread_rng();

    let state = chain.keys().choose(&mut rng).expect("The chain is empty!");
    return String::from(*state);
}