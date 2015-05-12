extern crate backend;

use backend::*;

fn main() {
    let genome = Genome::new(vec![
        Gene::Emitter(Emitter::new(0, 0.025)),
        Gene::Reaction(Reaction::new(ReactionType::Decay(Chemical::with_concentration(0, 0.02)), 4)),
        Gene::Receptor(Receptor::new(ReceptorType::LowerBound, 0, 1.0, 0.0)),
    ]);
    genome.save("basic.json").unwrap();
}
