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

    let mut states: HashMap<u32, Vec<State>> = HashMap::new();

    for factor in 1..23 {
        let state = State::new(factor, p);
        let these_states = states.entry(state.projlen()).or_default();
        these_states.push(state);
    }
    //lookahead_search(p, seed);
    search_best_first_parallel(states, 16, p);
}

 fn evaluate_candidate(candidate: &State, p: u8) -> u32 {
    let descendants = generate_descendants();
    let depth = 2;
    let mut best_projlen = u32::MAX;
    let mut layer: Vec<State> = vec![candidate.clone()];
    for layer_num in 0..depth {
        let mut new_layer: Vec<State> = Vec::new();
        for test_state in layer {
            let last_factor = test_state.factors.last().unwrap();
            for descendant in &descendants[last_factor] {
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

fn evaluate_candidate_mc(candidate: &State, p: u8, seed: u64) -> u32 {
    let descendants = generate_descendants();
    let depth = 1;

    let mut rng: rand_pcg::Lcg64Xsh32 = Pcg32::seed_from_u64(seed);
    let mut best_projlen = u32::MAX;
    let sample_count = 1000;
    for _ in 0..sample_count {
        let mut next_candidate = candidate.clone();
        for _ in 0..depth {
            let last_factor = next_candidate.factors.last().unwrap();
            let desc = &descendants[last_factor];
            let index = rng.gen_range(0..desc.len()-1);
            let chosen_desc = desc[index];
            next_candidate = next_candidate.append(chosen_desc, p)
        }
        if next_candidate.projlen() < best_projlen {
            best_projlen = next_candidate.projlen()
        }
    }
    best_projlen
}

fn evaluate_candidates(states: &[State], p: u8) -> Vec<u32> {
    states.iter().map(|state| evaluate_candidate(state, p)).collect()
}

fn lookahead_search(p: u8, seed: u64) {
    let descendants = generate_descendants();
    let mut candidates: Vec<State> = Vec::new();
    for factor in 1..23 {
        let state = State::new(factor, p);
        candidates.push(state);
    }
    let mut layer = 1;  
    let mut rng: rand_pcg::Lcg64Xsh32 = Pcg32::seed_from_u64(seed);
    let candidates_to_choose: usize = 15000;
    loop {
        let chunks: Vec<&[State]> = candidates.chunks(16).collect();
        let evaluations: Vec<u32> = {
            chunks
            .into_par_iter()
            .map(|candidates| evaluate_candidates(candidates, p))
            .flatten()
            .collect()
        };

        let mut all_evals: Vec<(&u32, State)> = evaluations.iter().zip(candidates).collect();
        all_evals.sort_by_key(|(eval, state)| *eval);
        if all_evals.first().unwrap().1.is_goal() {
            println!("{:?}", all_evals.first().unwrap().1.factors);
            return;
        }
        println!("Layer {}. Selected candidate with projlen {}. Best seen is {}.", layer, all_evals.first().unwrap().1.projlen(), all_evals.first().unwrap().0);
        layer += 1;
        let mut new_candidates: Vec<State> = Vec::new();
        let candidates_to_keep = if candidates_to_choose > evaluations.len() { evaluations.len() } else { candidates_to_choose };
        for (_, best_candidate) in all_evals.drain(..candidates_to_keep) {
            let last_factor = best_candidate.factors.last().unwrap();
            for descendant in &descendants[last_factor] {
                let new_state = best_candidate.append(*descendant, p);
                new_candidates.push(new_state);
            }

        }
        let rot = rng.gen_range(0..new_candidates.len());
        new_candidates.rotate_left(rot);
        candidates = new_candidates;
    }
}

fn run_to_fixed_limited(states: &[State], p: u8) -> (bool, HashMap<u32, Vec<State>>) {
    let descendants = generate_descendants();
    let mut result: HashMap<u32, Vec<State>> = HashMap::new();
    for state in states {
        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let new_state = state.append(*descendant, p);
            if new_state.is_goal() {
                //println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return (true, result);
            }
            let this_projlen = new_state.projlen();

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
                //println!("Draining {} from {}", i, to_add);
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
                    states.entry(*projlen).or_default().extend(result_states.drain(..should_add));
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
    let to_handle_per_layer: usize = 8000;
    let mut layer: i32 = 0;
    loop {
        // Split out
        let mut current_projlens: Vec<u32> = states.keys().into_iter().map(|x| *x).collect();
        current_projlens.sort();
        let mut have_handled: usize = 0;
        let mut collected: HashMap<u32, Vec<State>> = HashMap::new();
        println!("Layer {}. Truncated elements:", layer);
        layer += 1;

        for i in current_projlens {
            let mut have_added = 0;

            let mut highest_relevant_projlen = u32::MAX;
            let these_states = states.get_mut(&i).unwrap();
            let mut to_handle = these_states.len();
            if have_handled + to_handle > to_handle_per_layer {
                to_handle = to_handle_per_layer - have_handled;
            }
            let states_to_add = &mut these_states[..to_handle];

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
                    if *projlen >= highest_relevant_projlen {
                        continue;
                    }
                    have_added += result_states.len();
                    collected.entry(*projlen).or_default().append(result_states);
                    if have_added >= to_handle_per_layer {
                        let mut all_keys: Vec<u32> = collected.keys().into_iter().map(|x| *x).collect();
                        all_keys.sort();
                        let mut count_by_layer = 0;
                        let mut hit_highest_relevant = false;
                        for key in all_keys {
                            if hit_highest_relevant {
                                collected.remove(&key);
                            } else {
                                count_by_layer += collected.get(&key).unwrap().len();
                                
                                if count_by_layer >= to_handle_per_layer {
                                    highest_relevant_projlen = key;
                                    hit_highest_relevant = true;
                                }
                            }
                        }
                    }
                }
            }

            have_handled += to_handle;
            println!("{}: {}", i, to_handle);
            if have_handled == to_handle_per_layer {
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
            let last_factor = state.factors.last().unwrap();

            for descendant in &descendants[last_factor] {
                let new_state = state.append(*descendant, p);
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

fn search_best_first_limited_width(mut states: HashMap<u32, Vec<State>>, p: u8) {
    let descendants = generate_descendants();
    let mut lowest = *states.keys().min().unwrap();
    let mut highest = *states.keys().max().unwrap();
    let mut total_kept: usize = states.values().map(|x| x.len()).sum();
    let mut highest_seen_projlen = u32::MIN;
    const MAX_KEEP: usize = 60000;

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

        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let new_state = state.append(*descendant, p);
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
                let highest_states = states.get_mut(&highest).unwrap();
                highest_states.pop();
                total_kept -= 1;
                if highest_states.is_empty() {
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

        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let new_state = state.append(*descendant, p);
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
    pub factors: Vec<u32>,
    pub mat: Matrix,
}

impl State {
    pub fn new(factor: u32, p: u8) -> State {
        let eye = Matrix::identity(p);
        let mat: Matrix = act_by(&eye, factor, p);
        let projlen = mat.projlen();
        State {
            factors: vec![factor],
            mat,
        }
    }

    pub fn projlen(&self) -> u32 {
        return self.mat.projlen();
    }

    pub fn is_goal(&self) -> bool {
        self.projlen() == 1
    }

    pub fn append(&self, factor: u32, p: u8) -> State {
        let mut factors = self.factors.clone();
        factors.push(factor);
        let new_matrix: Matrix = act_by(&self.mat, factor, p);
        State {
            factors,
            mat: new_matrix,
        }
    }
}
