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
}
