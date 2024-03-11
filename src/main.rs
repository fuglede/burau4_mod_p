use hashbrown::HashMap;

use algebra::Matrix;
use garside::{generate_descendants, generate_matrix_map};
use core::panic;
use std::{env, os::linux::raw::stat, thread::current};
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

mod algebra;
mod garside;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: i32 = args[1].parse().unwrap();
    let seed: u64 = if args.len() >= 3 {
        args[2].parse().unwrap()
    } else { 0 };

    let beam_width: u64 = if args.len() >= 4 {
        args[3].parse().unwrap()
    } else { 250000 };
    println!("Starting search for kernel elements of Burau mod {}. Random seed: {}. Beam width: {}", p, seed, beam_width);


    let matrix_map = generate_matrix_map(p);

    let mut states: HashMap<i32, Vec<State>> = HashMap::new();

    for i in 1..23 {
        let matrix = matrix_map[&i].clone();
        let factors = vec![i];
        let state = State::new(factors, matrix);
        let these_states = states.entry(state.projlen()).or_default();
        these_states.push(state);
    }
    search_best_first_parallel(states, 16, p);
}

fn run_to_fixed_limited(mut states: &[State], p: i32) -> (bool, HashMap<i32, Vec<State>>) {
    let matrix_map = generate_matrix_map(p);
    let descendants = generate_descendants();
    let mut result: HashMap<i32, Vec<State>> = HashMap::new();
    for state in states {
        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let matrix = matrix_map[descendant].clone();
            let new_state = state.append(*descendant, matrix);
            let this_projlen = new_state.projlen();

            if this_projlen == 1 {
                //println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return (true, result);
            }
            let states_with_projlen = result.entry(this_projlen).or_default();
            states_with_projlen.push(new_state);
        }
    }
    (false, result)
}

fn search_best_first_parallel(mut states: HashMap<i32, Vec<State>>, num_threads: usize, p: i32) {
    // p = 2: 5
    // p = 3: 5000
    // p = 5: 150000
    let todo: usize = 150000;
    let tohandle: usize = 20000;
    let mut layer: i32 = 0;
    loop {
        // Split out
        let mut current_projlens: Vec<i32> = states.keys().into_iter().map(|x| *x).collect();
        current_projlens.sort();
        let mut have_added: usize = 0;
        layer += 1;
        for i in current_projlens {
            let these_states = states.get_mut(&i).unwrap();
            let mut to_add = these_states.len();
            if have_added + to_add > todo {
                to_add = todo - have_added;
                println!("Draining {} from {}", i, to_add);
                these_states.drain(to_add..);
                if these_states.is_empty() {
                    states.remove(&i);
                }
            }
            have_added += to_add;
        }
        println!("Finished layer {}. Projlen distribution for next layer:", layer);
        let mut res: Vec<i32> =  states.keys().into_iter().map(|x| *x).collect();
        res.sort();

        for k in res {
            println!("{}: {}", k, states.get_mut(&k).unwrap().len());
        }
        let current_projlen = *states.keys().into_iter().min().unwrap();
        println!("Handling layer {}", current_projlen);

        let states_to_handle = states.get_mut(&current_projlen).unwrap();
        let tohandlethis = if states_to_handle.len() > tohandle { tohandle } else { states_to_handle.len() };
        let chunks: Vec<&[State]> = states_to_handle[..tohandlethis].chunks(num_threads).collect();
        let results: Vec<(bool, HashMap<i32, Vec<State>>)> = chunks.into_par_iter().map(|chunk| run_to_fixed_limited(chunk, p)).collect();
        states_to_handle.drain(..tohandlethis);
        if (states_to_handle.is_empty()) {
            states.remove(&current_projlen);
        }
        for mut result in results {
            if result.0 {
                return;
            }
            for (projlen, result_states) in result.1.iter_mut() {
                states.entry(*projlen).or_default().append(result_states);
            }
        }
    }
}


fn search_best_first_parallel2(mut states: HashMap<i32, Vec<State>>, num_threads: usize, p: i32) {
    let todo: usize = 1500000;
    let mut layer: i32 = 0;
    loop {
        // Split out
        let mut current_projlens: Vec<i32> = states.keys().into_iter().map(|x| *x).collect();
        current_projlens.sort();
        let mut have_added: usize = 0;
        let mut collected: HashMap<i32, Vec<State>> = HashMap::new();
        println!("Layer {}. Truncated elements:", layer);
        layer += 1;

        for i in current_projlens {
            let these_states = states.get_mut(&i).unwrap();
            let mut to_add = these_states.len();
            if have_added + to_add > todo {
                to_add = todo - have_added;
            }
            let states_to_add = &mut these_states[..to_add].to_vec();


            let chunks: Vec<&[State]> = states_to_add.chunks(num_threads).collect();
            let results: Vec<(bool, HashMap<i32, Vec<State>>)> = chunks.into_par_iter().map(|chunk| run_to_fixed_limited(chunk, p)).collect();
            for mut result in results {
                if result.0 {
                    return;
                }
                for (projlen, result_states) in result.1.iter_mut() {
                    collected.entry(*projlen).or_default().append(result_states);
                }
            }

            have_added += to_add;
            println!("{}: {}", i, to_add);
            if have_added == todo { break; }
        }

        states = collected;
    }
}

fn search_beam(mut states: HashMap<i32, Vec<State>>, matrix_map: HashMap<i32, Matrix>, seed: u64, beam_width: u64) {
    let descendants = generate_descendants();
    let mut layer_num = 1;
    let mut rng = Pcg32::seed_from_u64(seed);

    loop {
        let mut next_layer: HashMap<i32, Vec<State>> = HashMap::new();
        let mut total_kept = 0;
        let mut highest = i32::MIN;
        let mut seen_num_by_projlen: HashMap<i32, i32> = HashMap::new();

        for state in states.values().flatten() {
            let last_factor = state.factors.last().unwrap();

            for descendant in &descendants[last_factor] {
                let matrix = matrix_map[descendant].clone();
                let new_state = state.append(*descendant, matrix);
                let this_projlen = new_state.projlen();

                if this_projlen == 1 {
                    println!("Found kernel element. Garside generators:");
                    println!("{:?}", new_state.factors);
                    return;
                }
                if this_projlen > highest && total_kept >= beam_width {
                    continue;
                }

                *seen_num_by_projlen.entry(this_projlen).or_default() += 1;

                if this_projlen > highest {
                    highest = this_projlen;
                }
                if total_kept < beam_width {
                    let states_with_projlen = next_layer.entry(this_projlen).or_default();
                    states_with_projlen.push(new_state);
                    total_kept += 1;
                } else {
                    if this_projlen < highest {
                        let states_with_projlen = next_layer.entry(this_projlen).or_default();
                        states_with_projlen.push(new_state);
                        let highest_states = next_layer.get_mut(&highest).unwrap();
                        highest_states.pop();
                        if highest_states.is_empty() {
                            next_layer.remove(&highest);
                            highest = *next_layer.keys().max().unwrap();
                        }
                    } else { // this_projlen == highest
                        let states_with_projlen = next_layer.entry(this_projlen).or_default();
                        let x = rng.gen_range(0..states_with_projlen.len());
                        if x < states_with_projlen.len() {
                            let _ = std::mem::replace(&mut states_with_projlen[x], new_state);
                        }
                    }
                }
            }
        }
        states = next_layer;
        println!("Finished layer {}. Projlen distribution for next layer:", layer_num);
        let mut res: Vec<i32> =  states.keys().into_iter().map(|x| *x).collect();
        res.sort();

        for k in res {
            println!("{}: {}", k, states.get_mut(&k).unwrap().len());
        }
        layer_num += 1;
    }
}

fn search_best_first_limited_search(mut states: HashMap<i32, Vec<State>>, matrix_map: HashMap<i32, Matrix>, seed: u64) {
    let descendants = generate_descendants();
    let mut lowest = *states.keys().min().unwrap();
    let mut highest = *states.keys().max().unwrap();
    let mut total_kept: usize = states.values().map(|x| x.len()).sum();
    let mut highest_seen_projlen = i32::MIN;
    let mut rng = Pcg32::seed_from_u64(seed);
    const MAX_KEEP: usize = 1000000;


    loop {
        let these_states = if states.get_mut(&lowest).is_some() {
            states.get_mut(&lowest).unwrap()
        } else {
            println!("lowest: {}, highest: {}, {:?}", lowest, highest, states.keys());
            panic!("boo3");
        };
        let state_opt = these_states.pop();
        total_kept -= 1;
        let state: State = if state_opt.is_some() {
            state_opt.unwrap()
        } else {
            println!("lowest failed");
            println!("lowest: {}, highest: {}", lowest, highest);
            panic!("boo");
        };
        if these_states.is_empty() {
            //println!("removing {}", lowest);
            states.remove(&lowest);
            lowest = if states.keys().min().is_some() {
                *states.keys().min().unwrap()
            } else {
                println!("lowest: {}, highest: {}, {:?}", lowest, highest, states.keys());
                panic!("boo2");
            };
            //println!("new lowest: {}", lowest);
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

            if this_projlen == 1 {
                println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return;
            }
            if this_projlen > highest && total_kept >= MAX_KEEP {
                continue;
            }

            let states_with_projlen = states.entry(this_projlen).or_default();
            states_with_projlen.push(new_state);
            if this_projlen < lowest {
                lowest = this_projlen;
            }
            if this_projlen > highest {
                highest = this_projlen;
            }

            total_kept += 1;
            if total_kept >= MAX_KEEP {
                let highest_states_opt = states.get_mut(&highest);//.unwrap();
                if highest_states_opt.is_some() {
                    let highest_states = highest_states_opt.unwrap();
                    highest_states.pop();
                    total_kept -= 1;
                    if highest_states.is_empty() {
                        //println!("removing {} (highest)", highest);
                        states.remove(&highest);
                        highest = *states.keys().max().unwrap();
                    }
                } else {
                    for (k, v) in states.iter_mut() {
                        println!("lowest: {}, highest: {}, val: {}, len: {}", lowest, highest, k, v.len());
                        panic!("boo");
                    }
                }
            }
        }
    }
}

fn search_best_first_reservoir(mut states: HashMap<i32, Vec<State>>, matrix_map: HashMap<i32, Matrix>, seed: u64) {
    let descendants = generate_descendants();
    let mut rng = Pcg32::seed_from_u64(seed);
    let mut lowest = *states.keys().min().unwrap();
    let mut highest_seen_projlen = i32::MIN;
    let mut num_seen_by_projlen: HashMap<i32, i32> = HashMap::new();


    loop {
        let these_states = states.get_mut(&lowest).unwrap();
        let state = these_states.pop().unwrap();
        if these_states.is_empty() {
            states.remove(&lowest);
            lowest = *states.keys().min().unwrap();
            if lowest > highest_seen_projlen {
                //println!("Now considering elements with projlen {}", lowest);
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
                //println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return;
            }
            let states_with_projlen = states.entry(this_projlen).or_default();
            let mut added = false;
            if states_with_projlen.len() < 50000 {
                states_with_projlen.push(new_state);
                added = true;
            } else {
                let x = rng.gen_range(0..states_with_projlen.len());
                if x < 50000 {
                    let _ = std::mem::replace(&mut states_with_projlen[x], new_state);
                    added = true;
                }
            }
            if added && this_projlen < lowest {
                lowest = this_projlen;
            }
        }
    }
}

#[derive(Clone)]
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
