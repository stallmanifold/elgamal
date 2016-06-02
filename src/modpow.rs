use num::{BigInt, Integer, Zero, One, Num};
//use std::ops::Mul;


pub trait ModPow {
    type IntType;

    fn mod_pow(base: &Self::IntType, exponent: &Self::IntType, modulus: &Self::IntType) -> Self::IntType;
}

/*
impl<T> ModPow for T where T: Num + Integer + Clone + Mul<&'a T> {
    type IntType = T;

    fn mod_pow(base: &T, exponent: &T, modulus: &T) -> T {
        let zero: T = Zero::zero();

        assert!(*modulus != zero);

        let one: T = <T as One>::one();
        let two: T = <T as One>::one() + <T as One>::one();

        if *modulus == one {
            return zero;
        }

        let mut result: T = <T as One>::one();
        let mut modded_base: T = base.mod_floor(modulus);
        let mut divided_exponent: T = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (result * modded_base).mod_floor(modulus);
            }
            divided_exponent = divided_exponent.div_floor(&two);
            modded_base = (modded_base * modded_base).mod_floor(modulus);
        }

        result
    }
}
*/

impl ModPow for BigInt {
    type IntType = BigInt;

    fn mod_pow(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
        let zero: BigInt = Zero::zero();

        assert!(*modulus != zero);

        let one: BigInt = One::one();
        let two: BigInt = <BigInt as One>::one() + <BigInt as One>::one();

        if *modulus == one {
            return zero;
        }

        let mut result: BigInt = One::one();
        let mut modded_base: BigInt = base.mod_floor(modulus);
        let mut divided_exponent: BigInt = exponent.clone();
        
        while divided_exponent > zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (result * &modded_base).mod_floor(modulus);
            }
            divided_exponent = divided_exponent.div_floor(&two);
            modded_base = (&modded_base * &modded_base).mod_floor(modulus);
        }

        result
    }
}