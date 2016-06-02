use num::{BigInt, Integer, Zero, One, Num};


pub trait ModPow {
    type IntType;

    fn mod_pow(base: &Self::IntType, exponent: &Self::IntType, modulus: &Self::IntType) -> Self::IntType;
}

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
