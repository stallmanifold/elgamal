#![allow(dead_code)]
use num::{BigInt, Integer};
use num::bigint::{RandBigInt, Sign};
use rand;
use modpow::ModPow;


type CipherText = (BigInt, BigInt);
type PlainText  = BigInt;

#[derive(Clone)]
pub struct GroupDescription {
    p: BigInt,
    g: BigInt,
    bit_size: usize,
}

impl GroupDescription {
    fn new(p: BigInt, g: BigInt) -> GroupDescription {
        let bit_size = p.bits();

        GroupDescription {
            p: p,
            g: g,
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
    fn new(group: GroupDescription, g: BigInt, key: BigInt) -> PublicKey {
        PublicKey {
            group: group,
            g: g,
            key: key,
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
    fn new(group: GroupDescription, g: BigInt, key: BigInt) -> PrivateKey {
        PrivateKey {
            group: group,
            g: g,
            key: key,
        }
    }
}

#[derive(Clone)]
pub struct KeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeyPair {
    fn new(private_key: PrivateKey, public_key: PublicKey) -> KeyPair {
        KeyPair {
            private_key: private_key,
            public_key: public_key,
        }
    }

    fn private_key(&self) -> PrivateKey {
        self.private_key.clone()
    }

    fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }
}

trait PublicKeyEncryptionScheme {
    fn generate(&mut self, desc: &GroupDescription)           -> KeyPair;
    fn encrypt(&mut self, message: &[u8], key: &PublicKey)    -> CipherText;
    fn decrypt(&self, message: &CipherText, key: &PrivateKey) -> PlainText;
}

pub struct ElGamal<R> {
    rng: R,
}

impl<R> ElGamal<R> where R: rand::Rng {
    fn new(rng: R) -> ElGamal<R> {
        ElGamal { 
            rng: rng,
        }
    }
}

impl<R> PublicKeyEncryptionScheme for ElGamal<R> where R: rand::Rng {
    fn generate(&mut self, desc: &GroupDescription) -> KeyPair {
        let lbound: BigInt = BigInt::from(1 as usize);
        let x: BigInt = RandBigInt::gen_bigint_range(&mut self.rng, &lbound, &desc.p);
        let h: BigInt = <BigInt as ModPow>::mod_pow(&desc.g, &x, &desc.p);

        let private_key = PrivateKey::new(desc.clone(), desc.g.clone(), x);
        let public_key  = PublicKey::new(desc.clone(), desc.g.clone(), h);

        KeyPair::new(private_key, public_key)
    }

    fn encrypt(&mut self, message: &[u8], key: &PublicKey) -> CipherText {
        let m = BigInt::from_bytes_be(Sign::Plus, message);
        let lbound = BigInt::from(1);
        let ubound = &key.group.p - BigInt::from(2);
        let nonce  = RandBigInt::gen_bigint_range(&mut self.rng, &lbound, &ubound);
        
        let gamma = <BigInt as ModPow>::mod_pow(&key.g, &nonce, &key.group.p);
        
        let mmp = m.mod_floor(&key.group.p);
        let ak  = <BigInt as ModPow>::mod_pow(&key.key, &nonce, &key.group.p);
        let delta = Integer::mod_floor(&(mmp*ak), &key.group.p);

        (gamma, delta)

    }

    fn decrypt(&self, cipher_text: &CipherText, key: &PrivateKey) -> PlainText {
        let gamma = &cipher_text.0;
        let delta = &cipher_text.1;
        let c = <BigInt as ModPow>::mod_pow(&gamma, &(&key.group.p - BigInt::from(1) - &key.key), &key.group.p);
        let m = Integer::mod_floor(&(c * delta), &key.group.p);

        m
    }
}
