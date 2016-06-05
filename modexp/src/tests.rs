use super::modexp::ModExp;
use num::{BigInt, Num};
use std::io::Write;
use std::io;


struct Test {
    data: Vec<TestCase>,
}

struct TestCase {
    modulus:  BigInt,
    base:     BigInt,
    exponent: BigInt,
    expected: BigInt,
}

// Test cases for small integers.
fn test_cases_small_integers() -> Test {
    Test {
        data: vec![
            TestCase {
                modulus:  BigInt::from(53),
                base:     BigInt::from(11),
                exponent: BigInt::from(13),
                expected: BigInt::from(52),
            },
            TestCase {
                modulus:  BigInt::from(17),
                base:     BigInt::from(4),
                exponent: BigInt::from(3),
                expected: BigInt::from(13),
            },
            TestCase {
                modulus:  BigInt::from(509),
                base:     BigInt::from(808),
                exponent: BigInt::from(454),
                expected: BigInt::from(9),
            },
            TestCase {
                modulus:  BigInt::from(610),
                base:     BigInt::from(131), 
                exponent: BigInt::from(870),
                expected: BigInt::from(1),
            },
            TestCase {
                modulus:  BigInt::from(802),
                base:     BigInt::from(923),
                exponent: BigInt::from(483),
                expected: BigInt::from(721),
            },
            TestCase {
                modulus:  BigInt::from(800),
                base:     BigInt::from(596),
                exponent: BigInt::from(240),
                expected: BigInt::from(576),
            },
            TestCase {
                modulus:  BigInt::from(833),
                base:     BigInt::from(365),
                exponent: BigInt::from(915),
                expected: BigInt::from(155),
            },
            TestCase {
                modulus:  BigInt::from(461),
                base:     BigInt::from(343), 
                exponent: BigInt::from(115),
                expected: BigInt::from(48),
            }
        ]
    }
}

// Test cases for computing modular powers on extremely large integers.
// These test cases were verified in Mathematica 10 using the PowerMod function.
fn test_cases_large_integers() -> Test {
    Test {
        data: vec![
            TestCase {
                modulus:  
                    Num::from_str_radix(
                        "749638037351852081151247094405133012235032062555866541611095018643\
                         535363413721770515177349316147941613894404099711872698372088686535\
                         683469192495021130492046334300628437246202382180495298481987987739\
                         372479006377400888623353111762276183648969728793051916182865825695\
                         65103627855665837810513319290012884326468813751", 10).unwrap(), 
                base:
                    Num::from_str_radix(
                        "164129383250808945874384406703678621596489217045151581900045748490\
                         868290229792470864829259786817694686625106454983835461186435041512\
                         388117934642968180231967384888123470927309027483636989889099899157\
                         094415936124189364693798303165153361235274685345850762442973852896\
                         986954817050473481072920056343116647695095169668", 10).unwrap(),
                exponent:
                    Num::from_str_radix(
                        "467400215328730823461841055834178866169807330531013031287757070008\
                         873010468651558887261418175887142301695860227816938321123655508023\
                         659244764594218274430021209092510615253487092025102388327057925146\
                         037348278553104651002893639903124774181594105388453460539419771598\
                         23262977569833661486606133857674031230191927456", 10).unwrap(),
                expected:
                    Num::from_str_radix(
                        "612054367420296281646438605074242800880369966540243212305161697955\
                         032632513642238079487655630058938577518349278909573088648068546700\
                         276914352025501358865503430800117920292868678278575721561270850533\
                         578286475591163803969873623890008903163860609693467969118912411728\
                         56097976636834454138308647614721353443994495382", 10).unwrap(),
            },
            TestCase {
                modulus:
                    Num::from_str_radix(
                        "150826454031439491816608041718464271500002050380870115404515300913\
                         741786003681086005673491004941492244761691622250175203899279707903\
                         772488587038009816740703358669769143220523889199795095564115581065\
                         837437960572241265819199493615155072848592668213693790132448044229\
                         968383223491543269394799975095570769191288963658", 10).unwrap(),
                base:
                    Num::from_str_radix(
                        "123681287711258573941928298099746725457517273478637683545060174216\
                         638262970483272570368833363244245971454887883510615019591739145539\
                         105551062049156866995364058694978537779953675129958481556024935442\
                         222023215903030699621740842023487697225318851203618966899076879186\
                         758821949294859609579013917868575868621684736005", 10).unwrap(),
                exponent:
                    Num::from_str_radix(
                        "546498713196540551582460974589564343088564268983688694262686617411\
                         025952749924788338768590141585082846875720148210620650851307474185\
                         250666854873484727670703432024875791768734484783831312980795875016\
                         478086087367762360110804632458445785486792492578144054515899993252\
                         84152127603137558146879284613544712519760740972", 10).unwrap(),
                expected:
                    Num::from_str_radix(
                        "125885984363343943191193201131927452146246752363546379964038034643\
                         255366601814203628322740323408858492526469561910696429955084277738\
                         070854414362281421634476222709715243401507043813221641902550851828\
                         053122865423507145383702325976412527857401026323052046960386245305\
                         632356218217258888544317496456278426315842632035", 10).unwrap(),
            },
            TestCase {
                modulus:  
                    Num::from_str_radix(
                        "381110944954956475722589135320618833712160651774651473116215557630\
                         125639689525595244544314838415569030409113863304132742833805692521\
                         261135210628882494522268117048187189002773666218168562717526809863\
                         325288810546408215621482501310732400335101655590468380233401045505\
                         56730888216193325481962887875598929690485588879", 10).unwrap(),
                base:     
                    Num::from_str_radix(
                        "178870466923655706196353171684311045413501506725461461186393751577\
                         971228714641656204297688330450709966122526002820158757486469553077\
                         461446037977759684623449209201009427371122326701206383359583793890\
                         573206558480000008273941984819380483767049368407740894633658873051\
                         802688990365364570293324603258682000610514960613", 10).unwrap(),
                exponent: 
                    Num::from_str_radix(
                        "248081652647414145680400965255877207255255612532308155761588504577\
                         705868136009905125725687326500511193013973799290598528779358989870\
                         218431785208716735310328228292269973283773236038413676531990589222\
                         409840984302436310260712888531917787511764707037528540018766221657\
                         45333467543903043513688710029208902915416129946", 10).unwrap(),
                expected: 
                    Num::from_str_radix(
                        "340914761053685877528412586743034863637315901148176884618040789181\
                         114439390936942459686825706806582390842177320591109244843557704987\
                         765657812222722894442756627940238763673858811820830831544793956876\
                         465991814554769783670230831408589002181850155545818712812203918323\
                         21129165814477648192094568360003920797284020021", 10).unwrap(),
            },
            TestCase {
                modulus:  Num::from_str_radix(
                    "7766034346301072300790391676096306800277788194151444813661672340007242\
                     0045970861735487656733112328510906430702048815592004831707832756291986\
                     4970676311052355549508063133853340511821688444867807319537727539798737\
                     8802382933576037744426453201138347772283178384120458007813753344181010\
                     7310760793742004539391981126065", 10).unwrap(),
                base:     Num::from_str_radix(
                    "9330027372731387333002480076297227685146675327692116638722007986806687\
                     0016509741417992098519242027288929828404011446330226326656597128183923\
                     8461099239545024064617423226890608788188790776517047871370668725379376\
                     0052174064951689840446060311175248945287928947644863774154460758951693\
                     5278380235360679191617952809513", 10).unwrap(),
                exponent: Num::from_str_radix(
                    "1569386106754055457802643214546151295669844703535223030594804805019394\
                     8619213185118412108496161592234959935883231241398679931981571701926618\
                     3548594793808709297109588739626678560735845565932799798672343868120031\
                     0759663778270405876758559097484512283373818805800968381218769274223885\
                     63434113674801763791987175064284", 10).unwrap(),
                expected: Num::from_str_radix(
                    "7805562712248983898952137503890329315828336921568195946339014941260691\
                     0858303867091599200485221631132287256645744577523323728465861867174886\
                     9980181961349136393281042204284082712738333005230483160058933501376078\
                     3469512743639969082602272053949879027788032892923293772915234816610958\
                     031511174499841416052548324201", 10).unwrap(),
            }
        ]
    }
}

fn test_cases_modulo_one() -> Test {
    Test {
        data: vec![
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(240),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus: BigInt::from(1),
                base:    Num::from_str_radix(
                    "9330027372731387333002480076297227685146675327692116638722007986806687\
                     0016509741417992098519242027288929828404011446330226326656597128183923\
                     8461099239545024064617423226890608788188790776517047871370668725379376\
                     0052174064951689840446060311175248945287928947644863774154460758951693\
                     5278380235360679191617952809513", 10).unwrap(),
                exponent: Num::from_str_radix(
                    "1569386106754055457802643214546151295669844703535223030594804805019394\
                     8619213185118412108496161592234959935883231241398679931981571701926618\
                     3548594793808709297109588739626678560735845565932799798672343868120031\
                     0759663778270405876758559097484512283373818805800968381218769274223885\
                     63434113674801763791987175064284", 10).unwrap(),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(-240),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(0),
                expected: BigInt::from(0),
            }
        ]
    }
}

fn run_test(test: &Test) {
    for test_case in test.data.iter() {
        let result = <BigInt as ModExp<&_>>::mod_exp(&test_case.base, &test_case.exponent, &test_case.modulus);
        assert_eq!(result, test_case.expected);
    }
}

#[test]
#[should_panic]
fn test_modexp_should_panic_with_zero_modulus() {
    let modulus  = BigInt::from(0);
    let exponent = BigInt::from(53);
    let base     = BigInt::from(11);

    let result = <BigInt as ModExp<&_>>::mod_exp(&base, &exponent, &modulus);

    assert_eq!(result, result);
    // mod_pow failed.
    assert!(false);
}

#[test]
fn test_run_small_positive_integers() {
    run_test(&test_cases_small_integers());
}

#[test]
fn test_run_large_positive_integers() {
    run_test(&test_cases_large_integers());
}

#[test]
fn test_run_integers_modulo_one() {
    run_test(&test_cases_modulo_one());
}

#[allow(dead_code)]
#[allow(unused_must_use)]
fn print_test_cases(test: &Test) {
    for test_case in test.data.iter() {
        let result = <BigInt as ModExp<&_>>::mod_exp(&test_case.base, &test_case.exponent, &test_case.modulus);
        writeln!(&mut io::stderr(), "\nmodulus:  {}", test_case.modulus);
        writeln!(&mut io::stderr(), "base:     {}", test_case.base);
        writeln!(&mut io::stderr(), "exponent: {}", test_case.exponent);
        writeln!(&mut io::stderr(), "expected: {}", test_case.expected);
        writeln!(&mut io::stderr(), "result:   {}\n", result);
    }
}

#[allow(dead_code)]
fn test_print_test_cases() {
    print_test_cases(&test_cases_large_integers());
}

struct TestCaseI {
    modulus:  isize,
    base:     isize,
    exponent: isize,
    expected: isize,
}

struct TestI {
    data: Vec<TestCaseI>,
}

// Test cases for small integers.
fn test_cases_small_integers_i() -> TestI {
    TestI {
        data: vec![
            TestCaseI {
                modulus:  53,
                base:     11,
                exponent: 13,
                expected: 52,
            },
            TestCaseI {
                modulus:  17,
                base:     4,
                exponent: 3,
                expected: 13,
            },
            TestCaseI {
                modulus:  509,
                base:     808,
                exponent: 454,
                expected: 9,
            },
            TestCaseI {
                modulus:  610,
                base:     131, 
                exponent: 870,
                expected: 1,
            },
            TestCaseI {
                modulus:  802,
                base:     923,
                exponent: 483,
                expected: 721,
            },
            TestCaseI {
                modulus:  800,
                base:     596,
                exponent: 240,
                expected: 576,
            },
            TestCaseI {
                modulus:  833,
                base:     365,
                exponent: 915,
                expected: 155,
            },
            TestCaseI {
                modulus:  461,
                base:     343, 
                exponent: 115,
                expected: 48,
            }
        ]
    }
}

fn run_test_i(test: &TestI) {
    for test_case in test.data.iter() {
        let result = <isize as ModExp<_>>::mod_exp(test_case.base, test_case.exponent, test_case.modulus);
        assert_eq!(result, test_case.expected);
    }
}

#[test]
fn test_run_small_positive_integers_i() {
    run_test_i(&test_cases_small_integers_i());
}
