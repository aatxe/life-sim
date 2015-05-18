use std::cell::Cell;
use std::collections::HashMap;
use rand::{Rand, Rng};

pub type Id = u8;
pub type Concentration = u8;
pub type LocusId = u8;
pub type LocusValue = u8;

pub struct ChemoBody {
    chems: HashMap<Id, Chemical>
}

impl ChemoBody {
    pub fn new() -> ChemoBody {
        ChemoBody { chems: HashMap::new() }
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

#[derive_Rand]
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


#[derive_Rand]
#[derive(RustcEncodable, RustcDecodable)]
pub enum IoType {
    Analogue,
    Digital,
}

#[derive_Rand]
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
}

impl Emitter {
    pub fn new(kind: IoType, chemical: Id, rate: u8, gain: Concentration, locus: LocusId,
               threshold: LocusValue, clear_after_read: bool, invert: bool) -> Emitter {
        Emitter { 
            kind: kind, chemical: chemical, rate: rate, gain: gain, locus: locus, 
            threshold: threshold, clear_after_read: clear_after_read, invert: invert,
        }
    }

    pub fn step(&self, body: &mut ChemoBody) {
        unimplemented!()
    }
}

#[derive_Rand]
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

impl Rand for Reaction {
    fn rand<R: Rng>(rng: &mut R) -> Reaction {
        Reaction { kind: rng.gen(), rate: rng.gen(), tick: Cell::new(0) }
    }
}

impl Reaction {
    pub fn new(kind: ReactionType, rate: u8) -> Reaction {
        Reaction { kind: kind, rate: rate, tick: Cell::new(0) }
    }

    pub fn step(&self, body: &ChemoBody) {
        unimplemented!()
    }
}

#[derive_Rand]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Receptor {
    kind: IoType,
    chemical: Id,
    nominal: LocusValue,
    gain: LocusValue,
    threshold: Concentration,
    invert: bool
}

impl Receptor {
    pub fn new(kind: IoType, chemical: Id, nominal: LocusValue, gain: LocusValue,
               threshold: Concentration, invert: bool) -> Receptor {
        Receptor {
            kind: kind, chemical: chemical, nominal: nominal, gain: gain, threshold: threshold,
            invert: invert
        }
    }

    pub fn step(&self, body: &ChemoBody) {
        unimplemented!()
    }
}
