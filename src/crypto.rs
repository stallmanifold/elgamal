use num::{Integer, Zero, One, Num};
use rand::Rng;
use modal::ModExp;


pub trait GroupDescription<F> where F: Clone + Integer + Num + Zero + One + ModExp<F, Output=F> {
    fn generator(&self) -> F;
    fn modulus(&self) -> F;
    fn bit_size(&self) -> usize;
}

#[derive(Clone)]
pub struct PublicKey<F, G> {
    group: G,
    key: F,
}

impl<F, G> PublicKey<F, G> where G: GroupDescription<F> {
    pub fn new(group: &G, g: &F, key: &F) -> PublicKey<F, G> {
        PublicKey {
            group: group.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey<F, G> {
    group: G,
    key: F,
}

impl PrivateKey<F, G> where G: GroupDescription<F> {
    pub fn new(group: &G, g: &F, key: &F) -> PrivateKey<F, G> {
        PrivateKey {
            group: group.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct KeyPair<F, G> {
    private_key: PrivateKey<F, G>,
    public_key: PublicKey<F, G>,
}

impl KeyPair<F, G> where G: GroupDescription<F> {
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

/*
pub trait RandGroupElement<F, G> where G: GroupDescription<F> {
    fn gen_group_element(&mut self, bit_size: usize) -> F;
    fn gen_group_range(&mut self, lbound: &F, ubound: &F) -> F;
}

impl<R, F, G> RandGroupElement<F, G> for R where G: GroupDescription<F> {
    
}
*/