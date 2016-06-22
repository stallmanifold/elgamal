#![allow(dead_code)]
use num::{BigInt, Integer};
use num::bigint::{RandBigInt, Sign};
use modal::ModExp;
use rand::Rng;


pub type CipherText = Vec<(BigInt, BigInt)>;
pub type PlainText  = Vec<u8>;

#[derive(Clone)]
pub struct GroupDescription {
    p: BigInt,
    g: BigInt,
    bit_size: usize,
}

impl GroupDescription {
    pub fn new(p: &BigInt, g: &BigInt) -> GroupDescription {
        let bit_size = p.bits();

        GroupDescription {
            p: p.clone(),
            g: g.clone(),
            bit_size: bit_size,
        }
    }
}

#[derive(Clone)]
pub struct PublicKey {
    group: GroupDescription,
    g:     BigInt,
    key:   BigInt,
}

impl PublicKey {
    pub fn new(group: &GroupDescription, g: &BigInt, key: &BigInt) -> PublicKey {
        PublicKey {
            group: group.clone(),
            g: g.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey {
    group: GroupDescription,
    g:     BigInt,
    key:   BigInt,
}

impl PrivateKey {
    pub fn new(group: &GroupDescription, g: &BigInt, key: &BigInt) -> PrivateKey {
        PrivateKey {
            group: group.clone(),
            g: g.clone(),
            key: key.clone(),
        }
    }
}

#[derive(Clone)]
pub struct KeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeyPair {
    pub fn new(private_key: PrivateKey, public_key: PublicKey) -> KeyPair {
        KeyPair {
            private_key: private_key,
            public_key: public_key,
        }
    }

    pub fn private_key(&self) -> PrivateKey {
        self.private_key.clone()
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }
}

#[inline]
fn decode(digit: &BigInt) -> Vec<u8> {
    BigInt::to_bytes_be(digit).1
}

#[inline]
fn encode(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, bytes)
}

#[inline]
fn __encrypt(digit: &BigInt, nonce: &BigInt, key: &PublicKey) -> (BigInt, BigInt) {      
    let gamma = <BigInt as ModExp<&_>>::mod_exp(&key.g, &nonce, &key.group.p);
        
    let mmp = digit.mod_floor(&key.group.p);
    let ak  = <BigInt as ModExp<&_>>::mod_exp(&key.key, &nonce, &key.group.p);
    let delta = Integer::mod_floor(&(mmp*ak), &key.group.p);

    (gamma, delta)
}

#[inline]
fn __decrypt(gamma: &BigInt, delta: &BigInt, key: &PrivateKey) -> BigInt {
    let c = <BigInt as ModExp<&_>>::mod_exp(&gamma, &(&key.group.p - BigInt::from(1) - &key.key), &key.group.p);
        
    Integer::mod_floor(&(c * delta), &key.group.p)
}

pub fn generate<R: Rng>(rng: &mut R, desc: &GroupDescription) -> KeyPair {
    let lbound: BigInt = BigInt::from(1);
    let x: BigInt = RandBigInt::gen_bigint_range(rng, &lbound, &desc.p);
    let h: BigInt = <BigInt as ModExp<&_>>::mod_exp(&desc.g, &x, &desc.p);

    let private_key = PrivateKey::new(&desc, &desc.g, &x);
    let public_key  = PublicKey::new(&desc, &desc.g, &h);

    KeyPair::new(private_key, public_key)
}

pub fn encrypt<R: Rng>(rng: &mut R, plain_text: &[u8], key: &PublicKey) -> CipherText {
    let lbound = BigInt::from(1);
    let ubound = &key.group.p - BigInt::from(2);
    let nonce  = RandBigInt::gen_bigint_range(rng, &lbound, &ubound);

    let bits_per_digit  = key.group.p.bits() - 1;
    let bytes_per_digit = bits_per_digit / 8;

    let mut digits  = Vec::new();

    for chunk in plain_text.chunks(bytes_per_digit) {
        let digit = encode(chunk);
        let (gamma, delta) = __encrypt(&digit, &nonce, key);
        digits.push((gamma, delta));
    }

    digits
}

pub fn decrypt(cipher_text: &CipherText, key: &PrivateKey) -> PlainText {
    let mut plain_text = Vec::new();

    for &(ref gamma, ref delta) in cipher_text {
        let digit     = __decrypt(&gamma, &delta, key);
        let mut chunk = decode(&digit);
        plain_text.append(&mut chunk);
    }

    plain_text
}
