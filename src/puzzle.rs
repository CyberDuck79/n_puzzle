use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec;

use priority_queue::PriorityQueue;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

mod parsing;
use parsing::*;

mod printing;
pub use printing::get_printing;
use printing::*;

mod node;
use node::Node;

pub mod heuristics;
pub use heuristics::*;

pub mod search_types;
pub use search_types::*;

pub struct Puzzle {
    size: usize,
    grid: Vec<u8>,
    goal: Vec<(usize, usize, usize)>,
}

pub fn generate_puzzle() -> Puzzle {
    let size = get_size();
    let len = size * size;

    let mut grid: Vec<u8> = (0..len as u8).collect();
    grid.shuffle(&mut thread_rng());
    let goal = gen_goal(size);

    Puzzle { size, grid, goal }
}

pub fn parse_puzzle(content: String) -> Result<Puzzle, &'static str> {
    let mut lines = parse_lines(&content);

    let size: usize = match parse_size(&mut lines) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let grid = match parse_grid(&lines, size) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    let goal = gen_goal(size);

    Ok(Puzzle { grid, size, goal })
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Puzzle {
    fn start_node(&self, heuristic: &Heuristics) -> Node {
        Node {
            state: self.grid.clone(),
            g: 0,
            h: heuristic.compute(&self.grid, &self.goal, self.size),
            parent: None,
        }
    }

    fn path(&self, end_node: &Node) -> usize {
        let mut grids = vec![&end_node.state];

        let mut current = end_node;
        while let Some(parent) = &current.parent {
            grids.push(&parent.state);
            current = parent;
        }

        print_grids(&grids, self.size);
        grids.len() - 1
    }

    fn result(
        &self,
        end_node: &Node,
        time_complexity: usize,
        max_space_complexity: usize,
    ) -> String {
        let move_number = self.path(&end_node);
        format!(
            "Solution found with {} moves\nTime complexity: {}\nMaximum space complexity: {}",
            move_number, time_complexity, max_space_complexity
        )
    }

    pub fn solve(&self, heuristic: Heuristics, search_type: SearchTypes, printing: bool) -> String {
        let mut open_queue = PriorityQueue::new();
        let mut closed_list = BTreeSet::new();
        let mut time_complexity = 0;
        let mut max_space_complexity = 0;

        let start = self.start_node(&heuristic);
        let cost = search_type.cost(start.g, start.h);
        open_queue.push(Rc::new(start), cost);

        print_grid(&self.grid, self.size);
        println!("Solving puzzle...");
        while let Some((current, _)) = open_queue.pop() {
            time_complexity += 1;

            if current.h == 0 {
                return self.result(&current, time_complexity, max_space_complexity);
            }

            if printing {
                print_state(&current.state, current.g, current.h, self.size);
            }

            for mut neighbor in current.possible_moves(self.size) {
                if closed_list.contains(&calculate_hash(&neighbor.state)) {
                    continue;
                };

                neighbor.h = heuristic.compute(&neighbor.state, &self.goal, self.size);
                neighbor.parent = Some(Rc::clone(&current));

                if let Some((existing, _)) = &mut open_queue.get_mut(&neighbor) {
                    if neighbor.g < existing.g {
                        open_queue.remove(&neighbor);
                    } else {
                        continue;
                    }
                }
                let cost = search_type.cost(neighbor.g, neighbor.h);
                open_queue.push(Rc::new(neighbor), cost);
            }
            closed_list.insert(calculate_hash(&current.state));

            let space_complexity = open_queue.len() + closed_list.len();
            if space_complexity > max_space_complexity {
                max_space_complexity = space_complexity;
            }

            if printing {
                reset_term(self.size);
            }
        }
        String::from("No solution for this puzzle")
    }
}
