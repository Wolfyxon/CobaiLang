extern crate rand;

use rand::Rng;

pub enum Uncertain {
    False,
    Maybe,
    True
}

impl Uncertain {
    pub fn from_bool(b: bool) -> Self {
        if b {
            return Uncertain::True;
        } else {
            return Uncertain::False;
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Uncertain::True => true,
            Uncertain::False => false,
            
            _ => {
                let maybe: u8 = rand::thread_rng().gen();
                return maybe == 1;
            }
        }
    }
}
