extern crate backend;

use std::cmp::Ordering;
use std::iter::repeat;
use backend::*;

fn main() {
    let target = 300;
    let mut fit = Fitness(std::u32::MAX, Genome::new());
    while fit.0 != 0 {
        fit = evolve(fit.1, 1000, 2000, |ticks: u32, genome| {
            Fitness((target as i64 - ticks as i64).abs() as u32, genome)
        });
    }
    fit.1.save("evolved.json").unwrap();
}

fn evolve<F>(base: Genome, trials: usize, cap: u32, fitness: F) -> Fitness
where F: Fn(u32, Genome) -> Fitness {
    repeat(base).take(trials).map(|genome| {
        let mut creature = Creature::new();
        genome.init(&mut creature);
        for t in 0 .. cap {
            genome.step(&mut creature);
            if creature.age() == Age::Child { return fitness(t, genome) }
        }
        fitness(cap, genome)
    }).min().unwrap()
}

struct Fitness(u32, Genome);

impl PartialEq for Fitness {
    fn eq(&self, other: &Fitness) -> bool {
        self.0 == other.0
    }
}

impl Eq for Fitness {}

impl PartialOrd for Fitness {
    fn partial_cmp(&self, other: &Fitness) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Fitness {
    fn cmp(&self, other: &Fitness) -> Ordering {
        self.0.cmp(&other.0)
    }
}
