extern crate weighted_probability_rs;
use rand::thread_rng;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use weighted_probability_rs::*;

/// Represents a marble to be chosen
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Marble {
    Red,
    Blue,
    Green,
}

fn main() -> Result<(), Box<dyn Error>> {
    // We initialize our aliases by supplying
    let marble_bag = Alias::from_weighted_tuples(&[
        WeightedTuple::new(1, Marble::Red),
        WeightedTuple::new(2, Marble::Blue),
        WeightedTuple::new(3, Marble::Green),
    ])?;
    let mut rng = thread_rng();
    let mut marble_selection_counts = HashMap::new();
    let max_iterations = 1_000_000;
    for _ in 0..max_iterations {
        let chosen_marble = marble_bag.select(&mut rng);
        marble_selection_counts.entry(chosen_marble).or_insert(0u64);
        marble_selection_counts.insert(chosen_marble, marble_selection_counts[&chosen_marble] + 1);
    }
    println!(
        "Making {} selections resulted in {:?}",
        max_iterations, marble_selection_counts
    );
    Ok(())
}
