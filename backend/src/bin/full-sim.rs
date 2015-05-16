extern crate backend;
extern crate rand;

use std::collections::HashMap;
use std::iter::repeat;
use backend::*;
use rand::{Rng, thread_rng};

fn main() {
    let net = NeuralNet::new(256, 4, 10, 15);
    let mut genes: Vec<_> = thread_rng().gen_iter().take(10000).collect();
    genes.extend(vec![Gene::Brain(10, 15, net.get_weights())]);
    let genome = Genome::new(genes);
    genome.save("basic.json").unwrap();
    let mut map: ChemicalMap = HashMap::new();
    for n in 0..255 {
        map.insert(n, Chemical::new(n));
    }
    simulate_genome(8, genome, &mut map);
}

fn simulate_genome(steps: u32, genome: Genome, map: &mut ChemicalMap) {
    let mut net = None;
    for gene in genome.iter() {
        match *gene {
            Gene::Brain(h, npl, ref weights) => {
                net = NeuralNet::with_weights(256, 4, h, npl, &weights)
            },
            _ => ()
        }
    }
    for _ in 0..steps {
        let mut inputs: Vec<_> = repeat(0.0).take(256).collect();
        let mut deltas: DeltaMap = HashMap::new();
        for gene in genome.iter() {
            match *gene {
                Gene::Emitter(ref e) => e.step(&mut deltas),
                Gene::Reaction(ref r) => r.step(map, &mut deltas),
                Gene::Receptor(ref r) => if let Some(val) = r.step(map, &deltas) {
                    inputs[r.id() as usize] = val;
                },
                _ => ()
            }
        }
        map.apply(&deltas);
        if let Some(ref net) = net {
            println!("Creature is currently {}.", net.update(inputs).unwrap().value());
        }
    }
}

trait OutputExt {
    fn value(&self) -> &'static str;
}

impl OutputExt for Vec<f32> {
    fn value(&self) -> &'static str {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        if a > b && a > c && a > d {
            "eating"
        } else if b > a && b > c && b > d {
            "sleeping"
        } else if c > a && c > b && c > d {
            "pooping"
        } else {
            "hacking"
        }
    }
}
