extern crate backend;

use std::collections::HashMap;
use backend::*;

fn main() {
    let genome = Genome::new(vec![
        Gene::Emitter(Emitter::new(0, 0.125)),
        Gene::Reaction(Reaction::new(ReactionType::Decay(Chemical::with_concentration(0, 0.25)), 4)),
        Gene::Receptor(Receptor::new(ReceptorType::LowerBound, 0, 1.0, 0.3)),
    ]);
    genome.save("chem.json").unwrap();
    let mut map: ChemicalMap = HashMap::new();
    simulate_genome(8, genome, &mut map);
}

fn simulate_genome(steps: u32, genome: Genome, map: &mut ChemicalMap) {
    for _ in 0..steps {
        let mut deltas: DeltaMap = HashMap::new();
        for gene in genome.iter() {
            match *gene {
                Gene::Emitter(ref e) => e.step(&mut deltas),
                Gene::Reaction(ref r) => r.step(map, &mut deltas),
                Gene::Receptor(ref r) => if let Some(val) = r.step(map, &deltas) {
                    println!("Receptor for {} triggered with output {}.", r.id(), val);
                },
                _ => ()
            }
        }
        map.apply(&deltas);
        map.values().map(|v|
            println!("Chemical {} has concentration {}.", v.id(), v.concnt())
        ).count();
    }
}
