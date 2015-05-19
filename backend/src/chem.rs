use std::cell::Cell;
use std::cmp::min;
use std::collections::HashMap;
use genome::{Creature, LocusId, LocusValue};

pub type Id = u8;
pub type Concentration = u8;

pub struct ChemoBody {
    chems: HashMap<Id, Chemical>
}

impl ChemoBody {
    pub fn new() -> ChemoBody {
        ChemoBody { chems: HashMap::new() }
    }

    pub fn get(&mut self, id: Id) -> &Chemical {
        self.chems.entry(id).or_insert(Chemical::new(id))
    }

    pub fn concnt(&mut self, id: Id) -> u8 { 
        self.chems.entry(id).or_insert(Chemical::new(id)).concnt()
    }

    pub fn gain(&mut self, id: Id, amount: Concentration) -> bool {
        let val = self.chems.entry(id).or_insert(Chemical::new(id));
        if let Some(new) = val.concnt().checked_add(amount) {
            *val = Chemical::with_concentration(id, new);
            true
        } else if val.concnt() == 255 {
            false
        } else {
            *val = Chemical::with_concentration(id, 255);
            false
        }
    }

    pub fn lose(&mut self, id: Id, amount: Concentration) -> bool {
        let val = self.chems.entry(id).or_insert(Chemical::new(id));
        if let Some(new) = val.concnt().checked_sub(amount) {
            *val = Chemical::with_concentration(id, new);
            true
        } else {
            false
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
        Chemical { id: id, concentration: 0 }
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
pub enum IoType {
    Analogue,
    Digital,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Emitter {
    kind: IoType,
    chemical: Id,
    rate: u8,
    gain: Concentration,
    locus: LocusId,
    threshold: LocusValue,
    clear_after_read: bool,
    invert: bool,
    tick: Cell<u8>,
}

impl Emitter {
    pub fn new(kind: IoType, chemical: Id, rate: u8, gain: Concentration, locus: LocusId,
               threshold: LocusValue, clear_after_read: bool, invert: bool) -> Emitter {
        Emitter { 
            kind: kind, chemical: chemical, rate: rate, gain: gain, locus: locus, 
            threshold: threshold, clear_after_read: clear_after_read, invert: invert, 
            tick: Cell::new(0)
        }
    }

    pub fn step(&self, creature: &mut Creature) {
        self.tick.set(self.tick.get() + 1);
        if self.tick.get() < self.rate { return }
        self.tick.set(0);
        let signal = if self.invert { 
            255 - creature.get_locus(self.locus) 
        } else { 
            *creature.get_locus(self.locus) 
        };
        let mut body = creature.chemo_body_mut();
        match self.kind {
            IoType::Analogue => {
                let modifier = self.gain as f32 / 255.0;
                if signal >= self.threshold { 
                    let output = ((signal - self.threshold) as f32 * modifier) as u8;
                    body.gain(self.chemical, output);
                } else {
                    let output = ((self.threshold - signal) as f32 * modifier) as u8;
                    if !body.lose(self.chemical, output) {
                        let concnt = body.concnt(self.chemical);
                        body.lose(self.chemical, concnt);
                    }
                }
            },
            IoType::Digital => {
                body.gain(self.chemical, if signal >= self.threshold { self.gain } else { 0 });
            }
        }
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

    pub fn step(&self, creature: &mut Creature) {
        self.tick.set(self.tick.get() + 1);
        if self.tick.get() < self.rate { return }
        self.tick.set(0);
        match self.kind {
            ReactionType::Normal(ref a, ref b, ref c, ref d) => {
                let n = min(creature.chemo_body_mut().concnt(a.id) / a.concnt(),
                            creature.chemo_body_mut().concnt(b.id) / b.concnt()); 
                let mut update = |c: &Chemical, add: bool| {
                    if add {
                        creature.chemo_body_mut().gain(c.id, n * c.concnt())
                    } else {
                        creature.chemo_body_mut().lose(c.id, n * c.concnt())
                    }
                };
                update(a, false);
                update(b, false);
                update(c, true);
                update(d, true);
            },
            ReactionType::Fusion(ref a, ref b, ref c) => {
                let n = min(creature.chemo_body_mut().concnt(a.id) / a.concnt(),
                            creature.chemo_body_mut().concnt(b.id) / b.concnt()); 
                let mut update = |c: &Chemical, add: bool| {
                    if add {
                        creature.chemo_body_mut().gain(c.id, n * c.concnt())
                    } else {
                        creature.chemo_body_mut().lose(c.id, n * c.concnt())
                    }
                };       
                update(a, false);
                update(b, false);
                update(c, true);
            },
            ReactionType::Decay(ref a) => {
                let n = creature.chemo_body_mut().concnt(a.id) / a.concnt();
                creature.chemo_body_mut().lose(a.id, n * a.concnt());
            },
            ReactionType::Catalytic(ref a, ref b, ref c) => {
                let n = min(creature.chemo_body_mut().concnt(a.id) / a.concnt(),
                            creature.chemo_body_mut().concnt(b.id) / b.concnt()); 
                let mut update = |c: &Chemical, add: bool| {
                    if add {
                        creature.chemo_body_mut().gain(c.id, n * c.concnt())
                    } else {
                        creature.chemo_body_mut().lose(c.id, n * c.concnt())
                    }
                };
                update(b, false);
                update(c, true);
            },
            ReactionType::CatalyticBreakdown(ref a, ref b) => {
                let n = min(creature.chemo_body_mut().concnt(a.id) / a.concnt(),
                            creature.chemo_body_mut().concnt(b.id) / b.concnt()); 
                creature.chemo_body_mut().lose(b.id, n * b.concnt());
            },
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Receptor {
    kind: IoType,
    chemical: Id,
    locus: LocusId,
    nominal: LocusValue,
    gain: LocusValue,
    threshold: Concentration,
    invert: bool
}

impl Receptor {
    pub fn new(kind: IoType, chemical: Id, locus: LocusId, nominal: LocusValue, gain: LocusValue,
               threshold: Concentration, invert: bool) -> Receptor {
        Receptor {
            kind: kind, chemical: chemical, locus: locus, nominal: nominal, gain: gain,
            threshold: threshold, invert: invert
        }
    }

    pub fn step(&self, creature: &mut Creature) {
        let val = creature.chemo_body_mut().concnt(self.chemical);
        let r = if self.invert { -1 } else { 1 };
        let output = match self.kind {
            IoType::Analogue => {
                let modifier = self.gain as f32 / 255.0;
                self.nominal + (((val - self.threshold) as f32 * modifier) as u8 * r)
            },
            IoType::Digital => self.nominal + if val > self.threshold { self.gain } else { 0 } * r
        };
        creature.set_locus(self.locus, output);
    }
}
