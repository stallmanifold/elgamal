use super::elgamal;
use super::elgamal::{GroupDescription, PrivateKey, PublicKey, KeyPair};
use crypto::fortuna::Fortuna;
use rand::SeedableRng;
use num::BigInt;
//use num::bigint::Sign;
//use rustc_serialize::base64;


// Stock lorem ipsum text
const MSG: &'static str = 
                  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
                   eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut \
                   enim ad minim veniam, quis nostrud exercitation ullamco laboris \
                   nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor \
                   in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
                   nulla pariatur. Excepteur sint occaecat cupidatat non proident, \
                   sunt in culpa qui officia deserunt mollit anim id est laborum. \
                   Sed ut perspiciatis unde omnis iste natus error sit voluptatem \
                   accusantium doloremque laudantium, totam rem aperiam, eaque ipsa \
                   quae ab illo inventore veritatis et quasi architecto beatae vitae \
                   dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit \
                   aspernatur aut odit aut fugit, sed quia consequuntur magni dolores \
                   eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, \
                   qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, \
                   sed quia non numquam eius modi tempora incidunt ut labore et dolore \
                   magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis \
                   nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut \
                   aliquid ex ea commodi consequatur? Quis autem vel eum iure \
                   reprehenderit qui in ea voluptate velit esse quam nihil molestiae \
                   consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla \
                   pariatur?";

struct TestCase {
    public_key:  PublicKey, 
    private_key: PrivateKey,
    plain_text:  Vec<u8>,
}

struct Test {
    data: Vec<TestCase>,
}

fn key_pair() -> KeyPair {
    let modulus = BigInt::from(91744613);
    let gen     = BigInt::from(62);
    let group_desc = GroupDescription::new(&modulus, &gen); 
    let pkey    = BigInt::from(84666074);
    let skey    = BigInt::from(18451137);

    let public_key = PublicKey::new(&group_desc, &gen, &pkey);
    let private_key = PrivateKey::new(&group_desc, &gen, &skey);

    KeyPair::new(private_key, public_key)
}

fn rng() -> Fortuna {
    let seed: Vec<u8> = vec![0xE2, 0xCF, 0xB1, 0xB1, 0x99, 0x3F, 0x57, 0xD2, 
                             0xBF, 0x39, 0xDD, 0xA3, 0xE0, 0x77, 0xF1, 0xF6, 
                             0x0E, 0x5A, 0xD2, 0x82, 0xB8, 0x45, 0x9F, 0x1E, 
                             0xF7, 0x0F, 0xF3, 0x27, 0xB1, 0xE4, 0x16, 0x0E, 
                             0xA5, 0x9A, 0x33, 0xC5, 0x3F, 0xA4, 0x4F, 0x42, 
                             0xAD, 0xF9, 0x75, 0x21, 0x39, 0xAB, 0x55, 0x08, 
                             0x42, 0xB7, 0x83, 0xBB, 0x24, 0x2D, 0x91, 0xED, 
                             0x85, 0xA0, 0x35, 0xAD, 0x6F, 0x3E, 0x34, 0xB1, 
                             0x6A, 0x66, 0xD7, 0x19, 0xAB, 0xD7, 0x2F, 0xFE,
                             0x52, 0x15, 0x63, 0x01, 0x11, 0x8A, 0x72, 0x24, 
                             0x2A, 0xD9, 0x9A, 0x06, 0x7D, 0x83, 0x58, 0x46,
                             0x61, 0x10, 0x12, 0xEB, 0xFC, 0x14, 0x75, 0x4D, 
                             0x0B, 0x8C, 0x94, 0x0A, 0x9C, 0x9B, 0x98, 0x0D,
                             0xC1, 0x81, 0x55, 0x92, 0x3E, 0x8F, 0x86, 0x35, 
                             0xB2, 0x61, 0x5E, 0xBA, 0xA5, 0x15, 0x4A, 0x8E,
                             0x5C, 0xEE, 0x1B, 0x75, 0x37, 0xF7, 0x7C, 0xC9 
                        ];

    let rng: Fortuna = <Fortuna as SeedableRng<&[u8]>>::from_seed(seed.as_ref());

    rng
}

fn elgamal_test_cases() -> Test {
    let pair = key_pair();
    let public_key = pair.public_key();
    let private_key = pair.private_key();

    Test {
        data: vec![
            TestCase {
                public_key:  public_key,   
                private_key: private_key,
                plain_text:  Vec::from(MSG),
            }                       
        ]
    }
}

fn elgamal_generated_key_test_cases() -> Test {
    let mut rng: Fortuna = rng();
    let modulus = BigInt::from(91744613);
    let gen     = BigInt::from(62);
    let desc    = GroupDescription::new(&modulus, &gen);

    let key_pair1 = elgamal::generate(&mut rng, &desc);
    let key_pair2 = elgamal::generate(&mut rng, &desc);
    let key_pair3 = elgamal::generate(&mut rng, &desc);
    let key_pair4 = elgamal::generate(&mut rng, &desc);

    Test {
        data: vec![
            TestCase {
                public_key:  key_pair1.public_key(),
                private_key: key_pair1.private_key(),
                plain_text:  Vec::from(MSG),
            },
            TestCase {
                public_key:  key_pair2.public_key(),
                private_key: key_pair2.private_key(),
                plain_text:  Vec::from(MSG),
            },
            TestCase {
                public_key:  key_pair3.public_key(),
                private_key: key_pair3.private_key(),
                plain_text:  Vec::from(MSG),
            },
            TestCase {
                public_key:  key_pair4.public_key(),
                private_key: key_pair4.private_key(),
                plain_text:  Vec::from(MSG),
            }
        ]
    }
}

fn run_tests(test: &Test) {
    let mut rng = rng();

    for test_case in test.data.iter() {
        let cipher_text = elgamal::encrypt(&mut rng, test_case.plain_text.as_ref(), &test_case.public_key);
        let recovered_plain_text: Vec<u8> = elgamal::decrypt(&cipher_text, &test_case.private_key);

        assert_eq!(&test_case.plain_text, &recovered_plain_text);
    }
}

#[test]
fn test_elgamal() {
    run_tests(&elgamal_test_cases());
}

#[test]
fn test_elgamal_with_generated_keys() {
    run_tests(&elgamal_generated_key_test_cases());
}