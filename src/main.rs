mod markov_chain;
mod random_writer;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next(); // Just to jump the program name

    let file_name = args.next().expect("You need to inform a file!");
    let text = fs::read_to_string(file_name).expect("The file_name doesn't point a file!");

    let k = args.next().expect("You need to inform a value for 'k'");
    let k = k
        .parse::<usize>()
        .expect("The value for 'k' isn't a number!");

    let l = args.next().expect("You need to inform a value for 'l'");
    let l = l
        .parse::<usize>()
        .expect("The value for 'l' isn't a number!");

    let chain = markov_chain::build(&text, k);

    /* for (state, chars) in &chain {
        println!("{}: {}", state, chars);
    } */

    random_writer::write(chain, l);
}
