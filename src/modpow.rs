use num::{BigInt, Integer, Zero, One, Num};

pub trait ModPow<I> {
    fn mod_pow(base: &I, exponent: &I, modulus: &I) -> I;
}

impl<I> ModPow<I> for I where I: Integer + Num {
    fn mod_pow(base: &I, exponent: &I, modulus: &I) -> I {
        let zero: I = Zero::zero();
        let one: I  = One::one();
        let two: I  = one + one;

        assert!(modulus != &zero);

        if *modulus == one {
            return Zero::zero();
        }       

        let mut result = one;
        let mut modded_base = base.mod_floor(modulus);
        let mut divided_exponent = exponent;
        
        while divided_exponent > &zero {
            if divided_exponent.mod_floor(&two) == one {
                result = (result * modded_base).mod_floor(modulus);
            }
            divided_exponent = &divided_exponent.div_floor(&two);
            modded_base = (modded_base * modded_base).mod_floor(modulus);
        }

        result
    }
}