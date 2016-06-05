use num::{Integer, Zero, One, Num, BigInt};
use super::ModExp;
use test::Bencher;


fn mod_exp<'a>(base: &'a BigInt, exponent: &'a BigInt, modulus: &'a BigInt) -> BigInt {
    let zero = Zero::zero();

    assert!(*modulus != zero);

    let one = One::one();
    let two = BigInt::from(2);

    if *modulus == one {
        return zero;
    }

    let mut result: BigInt = One::one();
    let mut modded_base = base.mod_floor(modulus);
    let mut divided_exponent = exponent.clone();
        
    while divided_exponent > zero {
        if divided_exponent.mod_floor(&two) == one {
            result = (&result * &modded_base).mod_floor(modulus);
        }
        divided_exponent = divided_exponent >> 1;
        modded_base = (&modded_base * &modded_base).mod_floor(modulus);
    }

    assert!(result < *modulus);

    result
}

fn bench_test_case() -> (BigInt, BigInt, BigInt) {
    let modulus =  Num::from_str_radix(
                        "150826454031439491816608041718464271500002050380870115404515300913\
                         741786003681086005673491004941492244761691622250175203899279707903\
                         772488587038009816740703358669769143220523889199795095564115581065\
                         837437960572241265819199493615155072848592668213693790132448044229\
                         968383223491543269394799975095570769191288963658", 10).unwrap();
    let base =     Num::from_str_radix(
                        "123681287711258573941928298099746725457517273478637683545060174216\
                         638262970483272570368833363244245971454887883510615019591739145539\
                         105551062049156866995364058694978537779953675129958481556024935442\
                         222023215903030699621740842023487697225318851203618966899076879186\
                         758821949294859609579013917868575868621684736005", 10).unwrap();
    let exponent = Num::from_str_radix(
                        "546498713196540551582460974589564343088564268983688694262686617411\
                         025952749924788338768590141585082846875720148210620650851307474185\
                         250666854873484727670703432024875791768734484783831312980795875016\
                         478086087367762360110804632458445785486792492578144054515899993252\
                         84152127603137558146879284613544712519760740972", 10).unwrap();

    (base, exponent, modulus)
}

#[bench]
fn bench_mod_exp_no_clone(b: &mut Bencher) {
    let (base, exponent, modulus) = bench_test_case();

    b.iter(|| mod_exp(&base, &exponent, &modulus));
}

#[bench]
fn bench_mod_exp_clone(b: &mut Bencher) {
    let (base, exponent, modulus) = bench_test_case();

    b.iter(|| <BigInt as ModExp<&_>>::mod_exp(&base, &exponent, &modulus));
}