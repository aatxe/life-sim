extern crate backend;

use backend::*;

fn main() {
    let genome = Genome::new(vec![Gene::Brain(2, 2, vec![
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ])]);
    genome.save("brain.json").unwrap();
    simulate_genome(8, genome);
}

fn simulate_genome(steps: u32, genome: Genome) {
    for gene in genome.iter() {
        match *gene {
            Gene::Brain(hidden, npl, ref weights) =>
                if let Some(net) = NeuralNet::with_weights(2, 2, hidden, npl, &weights) {
                    for _ in 0 .. steps {
                        println!("{:?}", net.update(vec![0.1, 0.33]));
                    }
                },
            _ => ()
        }
    }
}
