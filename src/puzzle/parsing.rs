use console::Term;
use std::io;

pub fn get_size() -> usize {
    let term = Term::stdout();
    println!("Please input the size of the puzzle.");

    loop {
        let mut size = String::new();

        io::stdin()
            .read_line(&mut size)
            .expect("Failed to read line");

        let size: usize = match size.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                term.clear_last_lines(1).unwrap();
                eprint!("Please enter a number: ");
                continue;
            }
        };

        term.clear_last_lines(2).unwrap();
        return size;
    }
}

pub fn gen_goal(size: usize) -> Vec<(usize, usize, usize)> {
    let mut goal: Vec<(usize, usize, usize)> = Vec::with_capacity(size * size + 1);
    let mut max: usize = size;
    let mut min: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    goal.push((0, 0, 0));
    while max >= min {
        while x < max - 1 {
            x += 1;
            goal.push((y, x, x + y * size));
        }
        while y < max - 1 {
            y += 1;
            goal.push((y, x, x + y * size));
        }
        while x > min {
            x -= 1;
            goal.push((y, x, x + y * size));
        }
        min += 1;
        while y > min {
            y -= 1;
            goal.push((y, x, x + y * size));
        }
        max -= 1;
    }
    goal.insert(0, goal[size * size - 1]);
    goal.remove(size * size);
    goal
}

pub fn parse_lines<'a>(content: &'a String) -> Vec<&'a str> {
    content
        .lines()
        .filter_map(|line| match line.split_once("#") {
            None => Some(line),
            Some((line_content, _)) => {
                if line_content.len() > 0 {
                    Some(line_content)
                } else {
                    None
                }
            }
        })
        .collect()
}

pub fn parse_size(lines: &mut Vec<&str>) -> Result<usize, &'static str> {
    match lines[0].parse::<usize>() {
        Ok(val) => {
            lines.remove(0);
            Ok(val)
        }
        Err(_) => return Err("convert size failed"),
    }
}

pub fn parse_grid(lines: &Vec<&str>, size: usize) -> Result<Vec<u8>, &'static str> {
    if lines.len() != size {
        return Err("Not correct col size");
    }
    let mut grid = Vec::with_capacity(size * size);
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split.len() != size {
            return Err("Not correct row size");
        }
        for part in split {
            let piece = match part.parse::<u8>() {
                Ok(val) => val,
                Err(_) => return Err("convert piece failed"),
            };
            if grid.contains(&piece) {
                return Err("duplicate piece");
            }
            if piece as usize >= size * size {
                return Err("Number too high for size");
            }
            grid.push(piece);
        }
    }
    Ok(grid)
}
