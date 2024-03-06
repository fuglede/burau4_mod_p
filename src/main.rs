use hashbrown::HashMap;

use algebra::Matrix;
use garside::{generate_descendants, generate_matrix_map};
use std::env;
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng};

mod algebra;
mod garside;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: i32 = args[1].parse().unwrap();
    let seed: u64 = if args.len() == 3 {
        args[2].parse().unwrap()
    } else { 0 };
    println!("Starting search for kernel elements of Burau mod {}. Random seed: {}", p, seed);

    let mut rng = Pcg32::seed_from_u64(seed);

    let descendants = generate_descendants();
    let matrix_map = generate_matrix_map(p);

    let mut states: HashMap<i32, Vec<State>> = HashMap::new();

    for i in 1..23 {
        let matrix = matrix_map[&i].clone();
        let factors = vec![i];
        let state = State::new(factors, matrix);
        let these_states = states.entry(state.projlen()).or_default();
        these_states.push(state);
    }

    let mut lowest = *states.keys().min().unwrap();
    let mut highest_seen_projlen = i32::MIN;
    let mut num_seen_by_projlen: HashMap<i32, i32> = HashMap::new();


    loop {
        let these_states = states.entry(lowest).or_default();
        let state = these_states.pop().unwrap();
        if these_states.is_empty() {
            states.remove(&lowest);
            lowest = *states.keys().min().unwrap();
            if lowest > highest_seen_projlen {
                println!("Now considering elements with projlen {}", lowest);
                highest_seen_projlen = lowest;
            }
        }

        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let matrix = matrix_map[descendant].clone();
            let new_state = state.append(*descendant, matrix);
            let this_projlen = new_state.projlen();
            *num_seen_by_projlen.entry(this_projlen).or_default() += 1;

            if this_projlen == 1 {
                println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return;
            }
            let states_with_projlen = states.entry(this_projlen).or_default();
            let mut added = false;
            if states_with_projlen.len() < 50000 {
                states.entry(this_projlen).or_default().push(new_state);
                added = true;
            } else {
                let x = rng.gen_range(0..states_with_projlen.len());
                if x < 50000 {
                    let _ = std::mem::replace(&mut states_with_projlen[x], new_state);
                    added = true;
                }
            }
            if added {
                if this_projlen < lowest {
                    lowest = this_projlen;
                }
            }
        }
    }
}

pub struct State {
    pub factors: Vec<i32>,
    pub mat: Matrix,
}

impl State {
    pub fn new<'a>(factors: Vec<i32>, mat: Matrix) -> State {
        State { factors, mat }
    }

    pub fn projlen(&self) -> i32 {
        self.mat.projlen()
    }

    pub fn append(&self, factor: i32, mat: Matrix) -> State {
        let mut factors = self.factors.clone();
        factors.push(factor);
        let new_matrix: Matrix = &self.mat * &mat;
        State {
            factors,
            mat: new_matrix,
        }
    }
}
