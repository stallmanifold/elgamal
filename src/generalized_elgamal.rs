use num::{Integer, Zero, One, Num};
use rand::Rng;
use modal::ModExp;


pub trait GroupElem: Num + Integer + Zero + One + Clone {}

#[derive(Clone)]
pub struct GroupSpec<Elem> {
    gen: Elem,
    modulus: Elem,
}

impl<Elem> GroupSpec<Elem> where Elem: GroupElem {
    pub fn new(gen: &Elem, modulus: &Elem) -> GroupSpec<Elem> {
        GroupSpec {
            gen: gen.clone(),
            modulus: modulus.clone(),
        }
    }
}

pub trait CyclicGroup {
    type Elem: GroupElem + ModExp;

    fn group_spec(&self) -> GroupSpec<Self::Elem>;
}

#[derive(Clone)]
pub struct PublicKey<G> where G: Clone + CyclicGroup {
    group_spec: GroupSpec<G::Elem>,
    key: G::Elem,
}

impl<G> PublicKey<G> where G: Clone + CyclicGroup {
    pub fn new(group_spec: &GroupSpec<G::Elem>, key: &G::Elem) -> PublicKey<G> {
        PublicKey {
            group_spec: group_spec.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey<G> where G: Clone + CyclicGroup {
    group_spec: GroupSpec<G::Elem>,
    key: G::Elem,
}

impl<G> PrivateKey<G> where G: Clone + CyclicGroup {
    pub fn new(group_spec: &GroupSpec<G::Elem>, key: &G::Elem) -> PrivateKey<G> {
        PrivateKey {
            group_spec: group_spec.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct KeyPair<G> where G: Clone + CyclicGroup {
    private_key: PrivateKey<G>,
    public_key: PublicKey<G>,
}

impl<G> KeyPair<G> where G: Clone + CyclicGroup {
    pub fn new(private_key: PrivateKey<G>, public_key: PublicKey<G>) -> KeyPair<G> {
        KeyPair {
            private_key: private_key,
            public_key: public_key,
        }
    }

    pub fn private_key(&self) -> PrivateKey<G> {
        self.private_key.clone()
    }

    pub fn public_key(&self) -> PublicKey<G> {
        self.public_key.clone()
    }
}

pub trait RandGroupElem<R, G> 
    where R: Rng, 
          G: Clone + CyclicGroup
{
    fn gen_group_elem(rng: &mut R, bit_size: usize) -> G::Elem;
    fn gen_group_elem_range(rng: &mut R, lbound: &G::Elem, ubound: &G::Elem) -> G::Elem;
}

pub trait EncodeDecode<G> {
    fn encode(&[u8]) -> G;
    fn decode(digits: &G) -> Vec<u8>;
}

pub trait GenerateKeyPair<G> where G: Clone + CyclicGroup {
    fn generate_key_pair<R: Rng>(rng: &mut R, spec: &GroupSpec<G::Elem>) -> KeyPair<G>;
}

pub fn generate<R, G>(rng: &mut R, spec: &GroupSpec<G::Elem>) -> KeyPair<G> 
    where R: Rng,
          G: Clone + CyclicGroup + RandGroupElem<R, G>,
{
    let lbound: G::Elem = <G::Elem as One>::one();
    let x: G::Elem = <G as RandGroupElem<_,_>>::gen_group_elem_range(rng, &lbound, &spec.modulus);
    let h: G::Elem = <G::Elem as ModExp>::mod_exp(&spec.gen, &x, &spec.modulus);

    let private_key = PrivateKey::new(spec, &x);
    let public_key  = PublicKey::new(spec, &h);

    KeyPair::new(private_key, public_key)
}
