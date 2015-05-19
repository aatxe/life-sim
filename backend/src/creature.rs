use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use chem::ChemoBody;

pub type LocusId = u8;
pub type LocusValue = u8;

pub struct Creature {
    loci: RefCell<HashMap<LocusId, LocusValue>>,
    chem: RefCell<ChemoBody>,
}

impl Creature {
    pub fn new() -> Creature {
        Creature { loci: RefCell::new(HashMap::new()), chem: RefCell::new(ChemoBody::new()) }
    }

    pub fn get_locus(&self, id: LocusId) -> LocusValue {
        *self.loci.borrow_mut().entry(id).or_insert(0)
    }

    pub fn set_locus(&self, id: LocusId, value: LocusValue) {
        self.loci.borrow_mut().insert(id, value);
    }

    pub fn chemo_body(&self) -> Ref<ChemoBody> {
        self.chem.borrow()
    }

    pub fn chemo_body_mut(&self) -> RefMut<ChemoBody> {
        self.chem.borrow_mut()
    }

    fn get(&self, id: Locus) -> LocusValue {
        *self.loci.borrow_mut().entry(id as u8).or_insert(0)
    }

    pub fn is_alive(&self) -> bool {
       self.get(Locus::Death) == 0
    }

    pub fn age(&self) -> Age {
        if self.get(Locus::AgedToSenile) == 0 {
            Age::Senile
        } else if self.get(Locus::AgedToOld) == 0 {
            Age::Old
        } else if self.get(Locus::AgedToAdult) == 0 {
            Age::Adult
        } else if self.get(Locus::AgedToYouth) == 0 {
            Age::Youth
        } else if self.get(Locus::AgedToAdolescent) == 0 {
            Age::Adolescent
        } else if self.get(Locus::AgedToChild) == 0 {
            Age::Child
        } else {
            Age::Baby
        }
    }

    pub fn get_drive(&self, drive: Drive) -> LocusValue {
        self.get(match drive {
            Drive::Hunger => Locus::Hunger
        })
    }
}

#[repr(u8)]
enum Locus {
    Death            = 0,
    AgedToChild      = 1,
    AgedToAdolescent = 2,
    AgedToYouth      = 3,
    AgedToAdult      = 4,
    AgedToOld        = 5,
    AgedToSenile     = 6,
    Hunger           = 7,
}

pub enum Age {
    Baby,
    Child,
    Adolescent,
    Youth,
    Adult,
    Old,
    Senile
}

pub enum Drive {
    Hunger
}
