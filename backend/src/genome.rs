use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
use std::path::Path;
use std::slice::Iter;
use chem::{Emitter, Reaction, Receptor};
use rand::{Rand, Rng};
use rustc_serialize::json::{decode, encode};

#[derive(RustcEncodable, RustcDecodable)]
pub enum Gene {
    Emitter(Emitter),
    Reaction(Reaction),
    Receptor(Receptor),
    Brain(usize, usize, Vec<f32>),
}

impl Rand for Gene {
    fn rand<R: Rng>(rng: &mut R) -> Gene {
        match rng.gen_range(0, 3) {
            0 => Gene::Emitter(rng.gen()),
            1 => Gene::Reaction(rng.gen()),
            _ => Gene::Receptor(rng.gen()),
        }
    }
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
