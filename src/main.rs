use hashbrown::HashMap;

use algebra::Matrix;
use garside::{act_by, generate_descendants, generate_matrix_map};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use rayon::prelude::*;
use std::env;

mod algebra;
mod garside;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: u8 = args[1].parse().unwrap();
    let seed: u64 = if args.len() >= 3 {
        args[2].parse().unwrap()
    } else {
        0
    };

    let beam_width: u64 = if args.len() >= 4 {
        args[3].parse().unwrap()
    } else {
        250000
    };
    println!(
        "Starting search for kernel elements of Burau mod {}. Random seed: {}. Beam width: {}",
        p, seed, beam_width
    );

    let matrix_map = generate_matrix_map(p);

    let mut states: HashMap<u32, Vec<State>> = HashMap::new();

    for factor in 1..23 {
        let matrix = matrix_map[&factor].clone();
        let state = State::new(factor, matrix);
        let these_states = states.entry(state.projlen()).or_default();
        these_states.push(state);
    }
    //lookahead_search(p, seed);
    search_best_first_parallel(states, 16, p);
}

fn evaluate_candidate(candidate: State, p: u8) -> u32 {
    let descendants = generate_descendants();
    let depth = 4;
    let mut layer: Vec<State> = vec![candidate.clone()];
    let mut best_projlen = u32::MAX;
    for layer_num in 0..depth {
        let mut new_layer: Vec<State> = Vec::new();
        for test_state in layer {
            let last_factor = test_state.factor;
            for descendant in &descendants[&last_factor] {
                let new_state = test_state.append(*descendant, p);
                if layer_num == depth - 1 {
                    let this_projlen = new_state.projlen();
                    if layer_num == depth - 1 && this_projlen < best_projlen {
                        best_projlen = this_projlen;
                    }
                }
                new_layer.push(new_state);
            }
        }
        layer = new_layer;
    }
    best_projlen
}

fn lookahead_search(p: u8, seed: u64) {
    let matrix_map = generate_matrix_map(p);
    let descendants = generate_descendants();
    let mut candidates: Vec<State> = Vec::new();
    let mut rng = Pcg32::seed_from_u64(seed);
    for factor in 1..23 {
        let matrix = matrix_map[&factor].clone();
        let state = State::new(factor, matrix);
        candidates.push(state);
    }
    loop {
        let evaluations: Vec<u32> = candidates
            .clone()
            .into_par_iter()
            .map(|candidate| evaluate_candidate(candidate, p))
            .collect();
        let mut best_eval = u32::MAX;
        let mut best_eval_index = 0;
        let mut seen_with_best = 0;
        for (i, eval) in evaluations.iter().enumerate() {
            if eval < &best_eval {
                best_eval = *eval;
                best_eval_index = i;
                seen_with_best = 1;
            } else if eval == &best_eval {
                seen_with_best += 1;
                if rng.gen_range(0..seen_with_best) == 0 {
                    best_eval_index = i;
                }
            }
        }
        if best_eval == 1 {
            println!("DONE");
            return;
        }
        let best_candidate = candidates.get(best_eval_index).unwrap().clone();
        let mut new_candidates: Vec<State> = Vec::new();
        let last_factor = best_candidate.factor;
        assert_eq!(best_eval, *evaluations.iter().min().unwrap());
        println!("Selected candidate with projlen {}, factor {}. Best seen is {}, index {}, best eval {}", best_candidate.projlen(), best_candidate.factor, best_eval, best_eval_index, evaluations.iter().min().unwrap());
        for descendant in &descendants[&last_factor] {
            let new_state = best_candidate.append(*descendant, p);
            new_candidates.push(new_state);
        }
        candidates = new_candidates;
    }
}

fn run_to_fixed_limited(states: &[State], p: u8) -> (bool, HashMap<u32, Vec<State>>) {
    let descendants = generate_descendants();
    let mut result: HashMap<u32, Vec<State>> = HashMap::new();
    for state in states {
        let last_factor = state.factor; //s.last().unwrap();

        for descendant in &descendants[&last_factor] {
            let new_state = state.append(*descendant, p);
            let this_projlen = new_state.projlen();

            if this_projlen == 1 {
                //println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factor);
                return (true, result);
            }
            let states_with_projlen = result.entry(this_projlen).or_default();
            states_with_projlen.push(new_state);
        }
    }
    (false, result)
}

fn search_best_first_parallel(mut states: HashMap<u32, Vec<State>>, num_threads: usize, p: u8) {
    // p = 2: 5
    // p = 3: 5000
    // p = 5: 150000
    let todo: usize = 150000;
    let tohandle: usize = 20000;
    let mut layer: u32 = 0;
    loop {
        // Split out
        let mut current_projlens: Vec<u32> = states.keys().into_iter().map(|x| *x).collect();
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
        println!(
            "Finished layer {}. Projlen distribution for next layer:",
            layer
        );
        let mut res: Vec<u32> = states.keys().into_iter().map(|x| *x).collect();
        res.sort();

        for k in res {
            println!("{}: {}", k, states.get_mut(&k).unwrap().len());
        }
        let current_projlen = *states.keys().into_iter().min().unwrap();
        println!("Handling layer {}", current_projlen);

        let states_to_handle = states.get_mut(&current_projlen).unwrap();
        let len = states_to_handle.len();
        let tohandlethis = if len > tohandle { tohandle } else { len };
        let indexstart = states_to_handle.len() - tohandlethis;
        let chunks: Vec<&[State]> = states_to_handle[indexstart..].chunks(num_threads).collect();
        let results: Vec<(bool, HashMap<u32, Vec<State>>)> = chunks
            .into_par_iter()
            .map(|chunk| run_to_fixed_limited(chunk, p))
            .collect();
        if indexstart == 0 {
            states.remove(&current_projlen);
        } else {
            states_to_handle.drain(indexstart..);
        }
        let mut highest = *states.keys().into_iter().min().unwrap();
        let mut total_states: usize = states.values().map(|x| x.len()).sum();

        for mut result in results {
            if result.0 {
                return;
            }
            for (projlen, result_states) in result.1.iter_mut() {
                if total_states >= todo && projlen >= &highest {
                    continue;
                }
                if projlen == &highest {
                    let can_add = todo - total_states;
                    let should_add = if can_add < result_states.len() {
                        can_add
                    } else {
                        result_states.len()
                    };
                    states
                        .entry(*projlen)
                        .or_default()
                        .extend_from_slice(&result_states[..should_add]);
                    total_states += should_add;
                } else {
                    states.entry(*projlen).or_default().append(result_states);
                    total_states += result_states.len();
                }
                if projlen > &highest {
                    highest = *projlen;
                }
            }
        }
    }
}

fn beam_search_parallel(mut states: HashMap<u32, Vec<State>>, num_threads: usize, p: u8) {
    let todo: usize = 7000;
    let mut layer: i32 = 0;
    loop {
        // Split out
        let mut current_projlens: Vec<u32> = states.keys().into_iter().map(|x| *x).collect();
        current_projlens.sort();
        let mut have_added: usize = 0;
        let mut collected: HashMap<u32, Vec<State>> = HashMap::new();
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
            let results: Vec<(bool, HashMap<u32, Vec<State>>)> = chunks
                .into_par_iter()
                .map(|chunk| run_to_fixed_limited(chunk, p))
                .collect();
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
            if have_added == todo {
                break;
            }
        }

        states = collected;
    }
}

fn beam_search(mut states: HashMap<u32, Vec<State>>, seed: u64, beam_width: u64, p: u8) {
    let descendants = generate_descendants();
    let mut layer_num = 1;
    let mut rng = Pcg32::seed_from_u64(seed);

    loop {
        let mut next_layer: HashMap<u32, Vec<State>> = HashMap::new();
        let mut total_kept = 0;
        let mut highest = u32::MIN;
        let mut seen_num_by_projlen: HashMap<u32, u32> = HashMap::new();

        for state in states.values().flatten() {
            let last_factor = state.factor; //;s.last().unwrap();

            for descendant in &descendants[&last_factor] {
                let new_state = state.append(*descendant, p);
                let this_projlen = new_state.projlen();

                if this_projlen == 1 {
                    println!("Found kernel element. Garside generators:");
                    println!("{:?}", new_state.factor);
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
                    } else {
                        // this_projlen == highest
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
        println!(
            "Finished layer {}. Projlen distribution for next layer:",
            layer_num
        );
        let mut res: Vec<u32> = states.keys().into_iter().map(|x| *x).collect();
        res.sort();

        for k in res {
            println!("{}: {}", k, states.get_mut(&k).unwrap().len());
        }
        layer_num += 1;
    }
}

fn search_best_first_limited_search(mut states: HashMap<u32, Vec<State>>, seed: u64, p: u8) {
    let descendants = generate_descendants();
    let mut lowest = *states.keys().min().unwrap();
    let mut highest = *states.keys().max().unwrap();
    let mut total_kept: usize = states.values().map(|x| x.len()).sum();
    let mut highest_seen_projlen = u32::MIN;
    let mut rng = Pcg32::seed_from_u64(seed);
    const MAX_KEEP: usize = 1000000;

    loop {
        let these_states = states.get_mut(&lowest).unwrap();
        let state_opt = these_states.pop();
        total_kept -= 1;
        let state: State = state_opt.unwrap();
        if these_states.is_empty() {
            states.remove(&lowest);
            lowest = *states.keys().min().unwrap();
            if lowest > highest_seen_projlen {
                println!("Now considering elements with projlen {}", lowest);
                highest_seen_projlen = lowest;
            }
        }

        let last_factor = state.factor; //s.last().unwrap();

        for descendant in &descendants[&last_factor] {
            let new_state = state.append(*descendant, p);
            let this_projlen = new_state.projlen();

            if this_projlen == 1 {
                println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factor);
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
                let highest_states = states.get_mut(&highest).unwrap();
                highest_states.pop();
                total_kept -= 1;
                if highest_states.is_empty() {
                    //println!("removing {} (highest)", highest);
                    states.remove(&highest);
                    highest = *states.keys().max().unwrap();
                }
            }
        }
    }
}

fn search_best_first_reservoir(mut states: HashMap<u32, Vec<State>>, seed: u64, p: u8) {
    let descendants = generate_descendants();
    let mut rng = Pcg32::seed_from_u64(seed);
    let mut lowest = *states.keys().min().unwrap();
    let mut highest_seen_projlen = u32::MIN;
    let mut num_seen_by_projlen: HashMap<u32, u32> = HashMap::new();

    loop {
        let these_states = states.get_mut(&lowest).unwrap();
        let state = these_states.pop().unwrap();
        if these_states.is_empty() {
            states.remove(&lowest);
            lowest = *states.keys().min().unwrap();
            if lowest > highest_seen_projlen {
                highest_seen_projlen = lowest;
            }
        }

        let last_factor = state.factor; //s.last().unwrap();

        for descendant in &descendants[&last_factor] {
            let new_state = state.append(*descendant, p);
            let this_projlen = new_state.projlen();
            *num_seen_by_projlen.entry(this_projlen).or_default() += 1;

            if this_projlen == 1 {
                println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factor);
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
    // pub factors: Vec<i32>,
    pub factor: u32,
    pub mat: Matrix,
}

impl State {
    pub fn new<'a>(factor: u32, mat: Matrix) -> State {
        State { factor, mat }
    }

    pub fn projlen(&self) -> u32 {
        self.mat.projlen()
    }

    pub fn append(&self, factor: u32, p: u8) -> State {
        //let mut factors = self.factors.clone();
        //factors.push(factor);
        let new_matrix: Matrix = act_by(&self.mat, factor, p);
        State {
            factor,
            mat: new_matrix,
        }
    }
}
