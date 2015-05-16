extern crate backend;

use std::collections::HashMap;
use std::iter::repeat;
use backend::*;

fn main() {
    let net = NeuralNet::new(255, 4, 10, 15);
    let genome = Genome::new(vec![
        Gene::Emitter(Emitter::new(0, 0.125)),
        Gene::Reaction(Reaction::new(ReactionType::Decay(Chemical::with_concentration(0, 0.25)), 4)),
        Gene::Receptor(Receptor::new(ReceptorType::LowerBound, 0, 1.0, 0.3)),
        Gene::Brain(10, 15, net.get_weights()),
    ]);
    genome.save("basic.json").unwrap();
    let mut map: ChemicalMap = HashMap::new();
    simulate_genome(8, genome, &mut map);
}

fn simulate_genome(steps: u32, genome: Genome, map: &mut ChemicalMap) {
    let mut net = None;
    for gene in genome.iter() {
        match *gene {
            Gene::Brain(h, npl, ref weights) => {
                net = NeuralNet::with_weights(255, 4, h, npl, &weights)
            },
            _ => ()
        }
    }
    for _ in 0..steps {
        let mut inputs: Vec<_> = repeat(0.0).take(255).collect();
        let mut deltas: DeltaMap = HashMap::new();
        for gene in genome.iter() {
            match *gene {
                Gene::Emitter(ref e) => e.step(&mut deltas),
                Gene::Reaction(ref r) => r.step(map, &mut deltas),
                Gene::Receptor(ref r) => if let Some(val) = r.step(map, &deltas) {
                    println!("Receptor for {} triggered with output {}.", r.id(), val);
                    inputs[r.id() as usize] = val;
                },
                _ => ()
            }
        }
        map.apply(&deltas);
        if let Some(ref net) = net {
            println!("Output: {:?}", net.update(inputs));
        }
    }
}
