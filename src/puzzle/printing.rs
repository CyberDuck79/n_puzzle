use console::Term;
use std::io;
use std::thread;
use std::time::Duration;

pub fn print_grid(grid: &Vec<u8>, size: usize) {
    for y in 0..size {
        for x in 0..size {
            if x == 0 {
                print!("[ ");
            }
            let value = grid[x + y * size];
            if value > 0 {
                print!("{:2} ", value);
            } else {
                print!("   ");
            }
            if x == size - 1 {
                println!("]");
            }
        }
    }
}

pub fn print_grids(grids: &Vec<&Vec<u8>>, size: usize) {
    for grid in grids.iter().rev() {
        print_grid(grid, size);
        println!("");
    }
}

pub fn print_state(state: &Vec<u8>, g: u32, h: u32, size: usize) {
    print_grid(state, size);
    println!("g: {} | f: {}", g, g + h);
}

pub fn reset_term(size: usize) {
    thread::sleep(Duration::from_millis(10));
    Term::stdout().clear_last_lines(size + 1).unwrap();
}

pub fn get_printing() -> bool {
    let term = Term::stdout();
    println!("Did you want printing while puzzle solving ? (y/n)");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: bool = match choice.trim() {
            "y" => true,
            "n" => false,
            _ => {
                term.clear_last_lines(1).unwrap();
                eprint!("Please enter (y/n): ");
                continue;
            }
        };

        term.clear_last_lines(2).unwrap();
        return choice;
    }
}
