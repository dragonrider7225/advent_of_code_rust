use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    io::{self, Write},
    ops::Add,
};

/// Runs the A* search algorithm on `initial_state` using `heuristic` to estimate the remaining
/// distance. If this function returns `None`, then there is no path from `initial_state` to a
/// state for which `heuristic` returns 0.
///
/// # Type parameters
/// `S` is the type of the states.
/// `D` is the type of the distances between states.
/// `H` is the type of the heuristic.
/// `O` is the type of the value of the heuristic.
///
/// In general, `D` and `O` should usually be the same numerical type.
pub fn run_a_star_for_distance<S, D, H, O>(initial_state: S, mut heuristic: H) -> Option<D>
where
    S: AStarState<Distance = D> + Clone + Debug + Display + Eq + Hash,
    for<'a> &'a D: Add<O, Output = D> + Add<Output = D>,
    D: Add<Output = D> + Clone + Debug + Default + Ord,
    H: Heuristic<S, O>,
    O: Default + PartialEq,
{
    writeln!(io::stderr().lock(), "This implementation of the A* algorithm is not correct. Output is likely to be *near* the true answer but no guarantees are given.").expect("Coudln't write to stderr");
    let target_heuristic = O::default();
    let mut completed_states: HashMap<S, (Option<S>, D)> = HashMap::new();
    let mut states = HashMap::new();
    let mut least_state = None;
    states.insert(initial_state, (None, D::default()));
    let mut i = 0;
    let result = loop {
        {
            i += 1;
            if i == 1000 {
                dbg!(
                    states.len(),
                    // &states,
                    completed_states.len(),
                    // &completed_states
                );
                i = 0;
            }
        }
        let (best_state, (parent, current_distance)) = {
            let mut min = None;
            for (state, (_, actual_distance)) in states.iter() {
                let h = &D::default() + heuristic.value(state);
                match least_state {
                    None => least_state = Some((h, state.clone())),
                    Some((least_h, _)) if h < least_h => least_state = Some((h, state.clone())),
                    _ => {}
                }
                let current_distance = actual_distance + heuristic.value(state);
                match &min {
                    None => min = Some((current_distance, state)),
                    Some((min_distance, _)) => {
                        if &current_distance < min_distance {
                            min = Some((current_distance, state));
                        }
                    }
                }
            }
            match min {
                None => {
                    assert!(states.is_empty());
                    break None;
                }
                Some((_, state)) => {
                    let state = state.clone();
                    states.remove_entry(&state).unwrap()
                }
            }
        };
        completed_states.insert(best_state.clone(), (parent, current_distance.clone()));
        if heuristic.value(&best_state) == target_heuristic {
            println!("Found goal at {}", best_state);
            let mut s = best_state;
            while let Some((Some(parent), distance)) = completed_states.get(&s) {
                println!("Total distance {:?}", distance);
                println!("From {}", parent);
                s = parent.clone();
            }
            break Some(current_distance);
        }
        let neighbors = best_state.neighbors();
        neighbors
            .into_iter()
            .filter(|(_, state)| !completed_states.contains_key(state))
            .map(move |(distance, state)| (state, current_distance.clone() + distance))
            .for_each(|(state, distance)| {
                if !states.contains_key(&state) || distance < states[&state].1 {
                    states.insert(state, (Some(best_state.clone()), distance));
                }
            });
    };
    dbg!(completed_states.len());
    result
}

/// A state that can be used for the A* search algorithm.
pub trait AStarState: Sized {
    /// The type of the distance between two states.
    type Distance;

    /// All possible states that can be reached in one move from this state along with their
    /// distances from this state.
    fn neighbors(&self) -> Vec<(Self::Distance, Self)>;
}

/// A simple function that gives a general idea of how far the given state is from the goal.
pub trait Heuristic<S, O> {
    /// The actual heuristic function.
    fn value(&mut self, state: &S) -> O;
}

// Any function that can be called multiple times can be used as a heuristic.
impl<F, S, O> Heuristic<S, O> for F
where
    F: FnMut(&S) -> O,
{
    fn value(&mut self, data: &S) -> O {
        self(data)
    }
}
