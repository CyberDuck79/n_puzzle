use std::env;
use std::fs;
use std::process;

// new dependencies
mod puzzle;
use puzzle::*;

fn check_args(mut args: env::Args) -> Option<String> {
    args.next();
    args.next()
}

fn main() {
    let puzzle = match check_args(env::args()) {
        Some(filename) => match fs::read_to_string(filename) {
            Ok(content) => match parse_puzzle(content) {
                Ok(puzzle) => puzzle,
                Err(err) => {
                    eprintln!("Puzzle parsing error: {}", err);
                    process::exit(1);
                }
            },
            Err(err) => {
                eprintln!("File error: {}", err);
                process::exit(1);
            }
        },
        None => generate_puzzle(),
    };
    let heuristic: Heuristics = get_heuristic();
    let search_type: SearchTypes = get_search_type();
    let printing: bool = get_printing();
    println!("{}", puzzle.solve(heuristic, search_type, printing));
}
