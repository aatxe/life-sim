extern crate backend;

use backend::*;

fn main() {
    let genome = Genome::load("evolved.json").unwrap();
    let mut creature = Creature::new();
    genome.init(&mut creature);
    let mut age = creature.age();
    for n in 0 .. 600 {
        genome.step(&mut creature);
        if creature.age() != age {
            println!("Creature aged from {:?} to {:?} at t = {}.", age, creature.age(), n);
            age = creature.age();
        }
    }
}
