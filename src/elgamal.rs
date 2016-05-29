use num;

pub struct GroupDescription<T> {
    order: num::BigUint,
    gen:   T,
}

pub struct PublicKey<T> {
    group: GroupDescription<T>,
    gen: T,
    key: T,
}

pub struct PrivateKey<T> {
    group: GroupDescription<T>,
    gen:   T,
    key:   T,
}
