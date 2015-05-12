extern crate rustc_serialize;

use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
use std::path::Path;
use std::slice::Iter;
use rustc_serialize::json::{decode, encode};

pub type Id = u8;
pub type Concentration = f32;
pub type ChemicalMap = HashMap<Id, Chemical>;
pub type DeltaMap = HashMap<Id, Concentration>;

pub trait ChemMapExt {
    fn apply(&mut self, deltas: &DeltaMap);
}

impl ChemMapExt for ChemicalMap {
    fn apply(&mut self, deltas: &DeltaMap) {
        for (id, diff) in deltas.iter() {
            let val = self.entry(*id).or_insert(Chemical::new(*id));
            val.concentration += *diff;
            val.concentration = if val.concentration > 1.0 {
                1.0
            } else if val.concentration < 0.0 {
                0.0
            } else {
                val.concentration 
            };
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Chemical {
    id: Id,
    concentration: Concentration,
}

impl Chemical {
    pub fn new(id: Id) -> Chemical {
        Chemical { id: id, concentration: 0.0 }
    }

    pub fn with_concentration(id: Id, concentration: Concentration) -> Chemical {
        Chemical { id: id, concentration: concentration }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn concnt(&self) -> Concentration {
        self.concentration
    }
}


#[derive(RustcEncodable, RustcDecodable)]
pub struct Emitter {
    chemical: Id,
    gain: f32,
}

impl Emitter {
    pub fn new(chemical: Id, gain: f32) -> Emitter {
        Emitter { chemical: chemical, gain: gain }
    }

    pub fn step(&self, deltas: &mut DeltaMap) {
        let val = deltas.entry(self.chemical).or_insert(0.0);
        *val += self.gain;
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum ReactionType {
    /// A + B -> C + D
    Normal(Chemical, Chemical, Chemical, Chemical),
    /// A + B -> C
    Fusion(Chemical, Chemical, Chemical),
    /// A -> nothing
    Decay(Chemical),
    /// A + B -> A + C
    Catalytic(Chemical, Chemical, Chemical),
    /// A + B -> A
    CatalyticBreakdown(Chemical, Chemical),
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Reaction {
    kind: ReactionType,
    rate: u8,
    tick: Cell<u8>,
}

impl Reaction {
    pub fn new(kind: ReactionType, rate: u8) -> Reaction {
        Reaction { kind: kind, rate: rate, tick: Cell::new(0) }
    }

    pub fn step(&self, map: &ChemicalMap, deltas: &mut DeltaMap) {
        self.tick.set(self.tick.get() + 1);
        if self.tick.get() < self.rate { return }
        self.tick.set(0);
        match self.kind {
            ReactionType::Normal(ref a, ref b, ref c, ref d) => {
                let n = (map[&a.id].concentration / a.concentration)
                        .min(map[&b.id].concentration / b.concentration); 
                let mut update = |c: &Chemical, add: bool| {
                    let val = deltas.entry(c.id).or_insert(0.0);
                    if add {
                        *val += n * c.concentration
                    } else {
                        *val -= n * c.concentration
                    }
                };
                update(a, false);
                update(b, false);
                update(c, true);
                update(d, true);
            },
            ReactionType::Fusion(ref a, ref b, ref c) => {
                let n = (map[&a.id].concentration / a.concentration)
                        .min(map[&b.id].concentration / b.concentration); 
                let mut update = |c: &Chemical, add: bool| {
                    let val = deltas.entry(c.id).or_insert(0.0);
                    if add {
                        *val += n * c.concentration
                    } else {
                        *val -= n * c.concentration
                    }
                };                
                update(a, false);
                update(b, false);
                update(c, true);
            },
            ReactionType::Decay(ref a) => {
                let n = map[&a.id].concentration / a.concentration;
                let val = deltas.entry(a.id).or_insert(0.0);
                *val -= n * a.concentration;
            },
            ReactionType::Catalytic(ref a, ref b, ref c) => {
                let n = (map[&a.id].concentration / a.concentration)
                        .min(map[&b.id].concentration / b.concentration); 
                let mut update = |c: &Chemical, add: bool| {
                    let val = deltas.entry(c.id).or_insert(0.0);
                    if add {
                        *val += n * c.concentration
                    } else {
                        *val -= n * c.concentration
                    }
                };
                update(b, false);
                update(c, true);
            },
            ReactionType::CatalyticBreakdown(ref a, ref b) => {
                let n = (map[&a.id].concentration / a.concentration)
                        .min(map[&b.id].concentration / b.concentration); 
                let val = deltas.entry(b.id).or_insert(0.0);
                *val -= n * b.concentration;
            },
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum ReceptorType {
    /// Receptor triggers when concentration is below threshold.
    LowerBound,
    /// Receptor triggers when concentration is above threshold.
    UpperBound,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Receptor {
    kind: ReceptorType,
    chemical: Id,
    gain: f32,
    threshold: f32,
}

impl Receptor {
    pub fn new(kind: ReceptorType, chemical: Id, gain: f32, threshold: f32) -> Receptor {
        Receptor { kind: kind, chemical: chemical, gain: gain, threshold: threshold }
    }

    pub fn step(&self, map: &mut ChemicalMap, deltas: &DeltaMap) -> Option<f32> {
        let prev = map.entry(self.chemical).or_insert(Chemical::new(self.chemical)).concentration;
        let curr = prev + deltas.get(&self.chemical).map(|u| *u).unwrap_or(0.0);
        match self.kind {
            ReceptorType::LowerBound => if prev > self.threshold && curr < self.threshold {
                Some(curr * self.gain)
            } else {
                None   
            },
            ReceptorType::UpperBound => if prev < self.threshold && curr > self.threshold {
                Some(curr * self.gain)
            } else {
                None   
            },
        }
    }

    pub fn id(&self) -> Id {
        self.chemical
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum Gene {
    Emitter(Emitter),
    Reaction(Reaction),
    Receptor(Receptor),
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Genome {
    genes: Vec<Gene>
}

impl Genome {
    pub fn new(genes: Vec<Gene>) -> Genome {
        Genome { genes: genes }
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

    pub fn iter(&self) -> Iter<Gene> {
        self.genes.iter()
    }
}
