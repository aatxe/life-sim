use std::cell::Cell;
use std::collections::HashMap;

pub type Id = u8;
pub type Concentration = f32;
pub type ChemicalMap = HashMap<Id, Chemical>;
pub type DeltaMap = HashMap<Id, Concentration>;

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
}

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
                *val -= n * a.concentration
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
                *val -= n * b.concentration
            },
        }
    }
}

pub enum ReceptorType {
    /// Receptor triggers when concentration is below threshold.
    LowerBound,
    /// Receptor triggers when concentration is above threshold.
    UpperBound,
}

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

    pub fn step(&self, map: &ChemicalMap, deltas: &DeltaMap) -> Option<f32> {
        let prev = map[&self.chemical].concentration;
        let curr = prev - deltas.get(&self.chemical).map(|u| *u).unwrap_or(0.0);
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
}

pub enum Gene {
    Emitter(Emitter),
    Reaction(Reaction),
    Receptor(Receptor),
}

pub struct Genome {
    genes: Vec<Gene>
}

impl Genome {
    pub fn new(genes: Vec<Gene>) -> Genome {
        Genome { genes: genes }
    }
}
