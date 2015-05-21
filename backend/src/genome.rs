use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
use std::path::Path;
use chem::{Chemical, Emitter, Reaction, Receptor};
use creature::Creature;
use rand::{thread_rng, Rand, Rng};
use rustc_serialize::json::{decode, encode};

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub enum Gene {
    InitialState(Chemical),
    Emitter(Emitter),
    Reaction(Reaction),
    Receptor(Receptor),
    Brain(usize, usize, Vec<f32>),
}

impl Rand for Gene {
    fn rand<R: Rng>(rng: &mut R) -> Gene {
        match rng.gen_range(0, 4) {
            1 => Gene::InitialState(rng.gen()),
            2 => Gene::Emitter(rng.gen()),
            3 => Gene::Reaction(rng.gen()),
            _ => Gene::Receptor(rng.gen()),
        }
    }
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Genome {
    genes: Vec<Gene>
}

impl Genome {
    pub fn new() -> Genome {
        Genome { genes: Vec::new() }
    }
    
    pub fn from_genes(genes: Vec<Gene>) -> Genome {
        Genome { genes: genes }
    }

    pub fn mutate(mut self) -> Genome {
        let mut rng = thread_rng();
        let val = rng.gen_range(0, self.genes.len() + 1);
        if val == self.genes.len() {
            self.genes.push(rng.gen())
        } else {
            self.genes[val] = match self.genes[val] {
                Gene::InitialState(ref ch) => if rng.gen() {
                    Gene::InitialState(Chemical::with_concentration(ch.id(), rng.gen()))
                } else {
                    Gene::InitialState(Chemical::with_concentration(rng.gen(), ch.concnt()))
                },
                Gene::Emitter(ref e) => Gene::Emitter(match rng.gen_range(0, 8) {
                    1 => Emitter { kind: rng.gen(), .. e.clone() },
                    2 => Emitter { chemical: rng.gen(), .. e.clone() },
                    3 => Emitter { rate: rng.gen(), .. e.clone() },
                    4 => Emitter { gain: rng.gen(), .. e.clone() },
                    5 => Emitter { locus: rng.gen(), .. e.clone() },
                    6 => Emitter { threshold: rng.gen(), .. e.clone() },
                    7 => Emitter { clear_after_read: rng.gen(), .. e.clone() },
                    _ => Emitter { invert: rng.gen(), .. e.clone() },
                }),
                Gene::Reaction(ref r) => Gene::Reaction(if rng.gen() {
                    Reaction { kind: rng.gen(), .. r.clone() }
                } else {
                    Reaction { rate: rng.gen(), .. r.clone() }
                }),
                Gene::Receptor(ref r) => Gene::Receptor(match rng.gen_range(0, 7) {
                    1 => Receptor { kind: rng.gen(), .. r.clone() },
                    2 => Receptor { chemical: rng.gen(), .. r.clone() },
                    3 => Receptor { locus: rng.gen(), .. r.clone() },
                    4 => Receptor { nominal: rng.gen(), .. r.clone() },
                    5 => Receptor { gain: rng.gen(), .. r.clone() },
                    6 => Receptor { threshold: rng.gen(), .. r.clone() },
                    _ => Receptor { invert: rng.gen(), .. r.clone() },
                }),
                _ => panic!("Something went wrong: failed to mutate a gene.")
            };
        }
        self
    }

    pub fn load<T: AsRef<Path>>(path: T) -> Result<Genome> {
        let mut f = try!(File::open(path.as_ref()));
        let mut data = String::new();
        try!(f.read_to_string(&mut data));
        decode(&data).map_err(|_|
            Error::new(ErrorKind::InvalidInput, "Failed to decode genome.")
        )
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let mut f = try!(File::create(path.as_ref()));
        try!(f.write_all(try!(encode(self).map_err(|_|
            Error::new(ErrorKind::InvalidInput, "Failed to encode genome.")
        )).as_bytes()));
        f.flush()
    }

    pub fn init(&self, creature: &mut Creature) {
        for gene in self.genes.iter() {
            if let Gene::InitialState(ref c) = *gene {
                creature.chemo_body_mut().gain(c.id(), c.concnt());
            }
        }
    }

    pub fn step(&self, creature: &mut Creature) {
        for gene in self.genes.iter() {
            match *gene {
                Gene::Emitter(ref e) => e.step(creature),
                Gene::Reaction(ref r) => r.step(creature),
                Gene::Receptor(ref r) => r.step(creature),
                _ => ()
            }
        }
    }
}
