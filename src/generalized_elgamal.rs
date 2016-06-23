use num::{Integer, Zero, One, Num};
use rand::Rng;
use modal::ModExp;


pub trait Field<F>: Num + Integer + Zero + One + Clone + ModExp<F, Output=F> {}

pub trait GroupDescription<F> where F: Field<F> {
    fn generator(&self) -> F;
    fn modulus(&self) -> F;
    fn bit_size(&self) -> usize;
}

#[derive(Clone)]
pub struct PublicKey<F, G> {
    group_desc: G,
    key: F,
}

impl<F, G> PublicKey<F, G> where F: Field<F>, G: Clone + GroupDescription<F> {
    pub fn new(group_desc: &G, key: &F) -> PublicKey<F, G> {
        PublicKey {
            group_desc: group_desc.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey<F, G> {
    group_desc: G,
    key: F,
}

impl<F, G> PrivateKey<F, G> where F: Field<F>, G: Clone + GroupDescription<F> {
    pub fn new(group_desc: &G, key: &F) -> PrivateKey<F, G> {
        PrivateKey {
            group_desc: group_desc.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct KeyPair<F, G> {
    private_key: PrivateKey<F, G>,
    public_key: PublicKey<F, G>,
}

impl<F, G> KeyPair<F, G> where F: Field<F>, G: Clone + GroupDescription<F> {
    pub fn new(private_key: PrivateKey<F, G>, public_key: PublicKey<F, G>) -> KeyPair<F, G> {
        KeyPair {
            private_key: private_key,
            public_key: public_key,
        }
    }

    pub fn private_key(&self) -> PrivateKey<F, G> {
        self.private_key.clone()
    }

    pub fn public_key(&self) -> PublicKey<F, G> {
        self.public_key.clone()
    }
}


pub trait RandomGroupElement<F, G> where F: Field<F>, G: GroupDescription<F> {
    fn gen_group_element(&mut self, bit_size: usize) -> F;
    fn gen_group_element_range(&mut self, lbound: &F, ubound: &F) -> F;
}

/*
impl<R, F, G> RandGroupElement<F, G> for R 
    where R: Rng,
          F: Field<F>,
          G: GroupDescription<F> 
{
    fn gen_group_element(&mut self, bit_size: usize) -> F {

    }

    fn gen_group_element_range(&mut self, lbound: &F, ubound: &F) -> F {

    }
}
*/

pub trait EncodeDecode<F> {
    fn encode(&[u8]) -> F;
    fn decode(digit: &F) -> Vec<u8>;
}