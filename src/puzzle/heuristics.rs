use console::Term;
use std::io;

pub enum Heuristics {
    Hamming,
    Manhattan,
    LinearConflicts,
}

impl Heuristics {
    pub fn compute(&self, grid: &[u8], goal: &[(usize, usize, usize)], size: usize) -> u32 {
        match self {
            Heuristics::Hamming => Heuristics::hamming_h(grid, goal),
            Heuristics::Manhattan => Heuristics::manhattan_h(grid, goal, size),
            Heuristics::LinearConflicts => {
                Heuristics::manhattan_h(grid, goal, size)
                    + Heuristics::linear_conflicts(grid, goal, size)
            }
        }
    }

    fn hamming_h(grid: &[u8], goal: &[(usize, usize, usize)]) -> u32 {
        grid.iter()
            .enumerate()
            .map(|(index, piece)| {
                if *piece > 0 {
                    let (_, _, index_goal) = goal[*piece as usize];
                    if index != index_goal {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum()
    }

    fn manhattan_h(grid: &[u8], goal: &[(usize, usize, usize)], size: usize) -> u32 {
        grid.iter()
            .enumerate()
            .map(|(index, piece)| {
                if *piece > 0 {
                    let (y, x) = (index / size, index % size);
                    let (y_goal, x_goal, _) = goal[*piece as usize];
                    let mut score = 0;
                    if y > y_goal {
                        score += y - y_goal;
                    } else {
                        score += y_goal - y;
                    }
                    if x > x_goal {
                        score += x - x_goal;
                    } else {
                        score += x_goal - x;
                    }
                    score as u32
                } else {
                    0
                }
            })
            .sum()
    }

    fn linear_conflicts(grid: &[u8], goal: &[(usize, usize, usize)], size: usize) -> u32 {
        let mut linear_conflicts = 0;
        for y in 0..size {
            let pieces_in_good_line: Vec<(usize, usize)> = (0..size)
                .filter_map(|x| {
                    let piece = grid[x + y * size];
                    let (y_target, x_target, _) = goal[piece as usize];
                    match y == y_target {
                        true => Some((x, x_target)),
                        false => None,
                    }
                })
                .collect();
            if pieces_in_good_line.len() > 1 {
                for (i, piece1) in pieces_in_good_line.iter().enumerate() {
                    for piece2 in &pieces_in_good_line[i + 1..] {
                        if (piece1.0 < piece2.0 && piece1.1 >= piece2.0)
                            || (piece2.0 < piece1.0 && piece2.1 >= piece1.0)
                        {
                            linear_conflicts += 1;
                        }
                    }
                }
            }
        }
        for x in 0..size {
            let pieces_in_good_col: Vec<(usize, usize)> = (0..size)
                .filter_map(|y| {
                    let piece = grid[x + y * size];
                    let (y_target, x_target, _) = goal[piece as usize];
                    match x == x_target {
                        true => Some((y, y_target)),
                        false => None,
                    }
                })
                .collect();
            if pieces_in_good_col.len() > 1 {
                for (i, piece1) in pieces_in_good_col.iter().enumerate() {
                    for piece2 in &pieces_in_good_col[i + 1..] {
                        if (piece1.0 < piece2.0 && piece1.1 >= piece2.0)
                            || (piece2.0 < piece1.0 && piece2.1 >= piece1.0)
                        {
                            linear_conflicts += 1;
                        }
                    }
                }
            }
        }
        linear_conflicts * 2
    }
}

pub fn get_heuristic() -> Heuristics {
    let term = Term::stdout();
    println!("Please choose the heuristic function:");
    println!("1) Hamming");
    println!("2) Manhattan");
    println!("3) Manhattan with linear conflicts");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let heuristic: Heuristics = match choice.trim().parse() {
            Ok(num) => match num {
                1 => Heuristics::Hamming,
                2 => Heuristics::Manhattan,
                3 => Heuristics::LinearConflicts,
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
        return heuristic;
    }
}
