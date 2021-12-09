use console::Term;
use std::cmp::Reverse;
use std::io;

pub enum SearchTypes {
    Uniform,
    Greedy,
    Astar,
}

impl SearchTypes {
    pub fn cost(&self, g: u32, h: u32) -> Reverse<u32> {
        match self {
            SearchTypes::Uniform => Reverse(g),
            SearchTypes::Greedy => Reverse(h),
            SearchTypes::Astar => Reverse(g + h),
        }
    }
}

pub fn get_search_type() -> SearchTypes {
    let term = Term::stdout();
    println!("Please choose the search methods:");
    println!("1) Uniform");
    println!("2) Greedy");
    println!("3) Astar");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let search_type: SearchTypes = match choice.trim().parse() {
            Ok(num) => match num {
                1 => SearchTypes::Uniform,
                2 => SearchTypes::Greedy,
                3 => SearchTypes::Astar,
                _ => {
                    term.clear_last_lines(1).unwrap();
                    eprint!("Please enter a valid number: ");
                    continue;
                }
            },
            Err(_) => {
                term.clear_last_lines(1).unwrap();
                eprint!("Please enter a valid number: ");
                continue;
            }
        };

        term.clear_last_lines(5).unwrap();
        return search_type;
    }
}
