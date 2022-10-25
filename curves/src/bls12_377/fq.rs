// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_fields::{
    FftParameters,
    FieldParameters,
    Fp384,
    Fp384Parameters,
    PoseidonDefaultParameters,
    PoseidonDefaultParametersEntry,
};
use snarkvm_utilities::biginteger::BigInteger384 as BigInteger;

pub type Fq = Fp384<FqParameters>;

pub struct FqParameters;

impl Fp384Parameters for FqParameters {}

impl FftParameters for FqParameters {
    type BigInteger = BigInteger;

    #[rustfmt::skip]
    const POWERS_OF_ROOTS_OF_UNITY: &'static [BigInteger] = &[
        BigInteger([2022196864061697551, 17419102863309525423, 8564289679875062096, 17152078065055548215, 17966377291017729567, 68610905582439508]),
        BigInteger([1097558877362951325, 10990216524069526157, 1703764964909126124, 5337477398168494451, 8388789446441546832, 83780176967333390]),
        BigInteger([6021325183928272861, 3071743451473281050, 17793609045682579055, 4499483110946763311, 3694917660005904773, 35840689841924604]),
        BigInteger([4148421423508782473, 16486417951907259381, 13681320370461884655, 9543156358084556689, 15654115908807492478, 85178773442619605]),
        BigInteger([4440291260215189129, 8134083368238958870, 5850770822935137603, 18388642548453398432, 4033689956485938698, 96943229116346922]),
        BigInteger([13321959743753581084, 13957307756377123667, 8022065601003020126, 15105587473044942004, 3436102915385715417, 16497947308509307]),
        BigInteger([12740765449271843915, 13529810863058982449, 15988003695485397690, 5204094055301492127, 12204797504641322421, 54884696873855227]),
        BigInteger([7200341240129221640, 128866105844736666, 1972097482351781478, 7508123272393265315, 18134316519918384483, 90386983337599854]),
        BigInteger([11182828541229423812, 3352059565776569042, 745605881396983371, 10544136951768851167, 13944324842745283530, 76153606183323606]),
        BigInteger([5451839093323299025, 11344095019039321738, 3696612540108785619, 849808749100130193, 7737394061630388518, 41447732009599376]),
        BigInteger([4914435911578270728, 11442571726223078752, 200783168592117624, 5223206539372270413, 12471543125846282538, 14780043884677376]),
        BigInteger([6835942096487989968, 9916740166863458597, 15149601694743014865, 3251390487756549529, 16202626875757167903, 78637381314787701]),
        BigInteger([10024136201786834022, 2737410832464918544, 11569195233337582907, 6882399291950297037, 17956512286065493665, 87559046345038795]),
        BigInteger([7787906257689135971, 4007925209220880779, 5799259275421890332, 5651393587803319504, 5361114559272044762, 78591763064139593]),
        BigInteger([8749978828541193864, 7615081815133949072, 13070992874042179472, 1364907361524039183, 10109099343823847681, 104282651814875028]),
        BigInteger([16730838284865363583, 15400461195736119534, 4032519462999757033, 15551203984340620330, 382650037072032689, 76641657953301822]),
        BigInteger([8166159088317375095, 1704530912566839702, 13372111531284600631, 9715437978473519961, 4887487291063440487, 6286512958015546]),
        BigInteger([1768534497835661159, 1244838626041538416, 9153540930636989172, 12437852256056737422, 2519703766117714353, 37580752840997918]),
        BigInteger([13468864600499712370, 15009260878077565, 14291559461807896332, 9284492953958434183, 18315507429112290509, 41332741727179643]),
        BigInteger([8269976866043356680, 16766307433795223092, 339184897717558508, 9029309210784216270, 16947438621423748249, 1154969810162894]),
        BigInteger([10598383875620927263, 1261087173785984910, 11498737931675819777, 8568819297979713299, 1166319621891739467, 53485818962535716]),
        BigInteger([1592802484188722575, 16097103819145948294, 7922347041881118053, 14182139815326903761, 1080552542780789890, 84770178757176689]),
        BigInteger([17305426336106269005, 8862145762498523999, 13494918675279718853, 3717788643186382361, 10515687947609219968, 112422016356963276]),
        BigInteger([10874618176231306138, 13631539808973743734, 11959456582258358795, 13957665559292951976, 7355794120017380469, 51180796759257049]),
        BigInteger([9944918783779352295, 11674736477566853654, 4430181012666063989, 5365556697595744810, 13024803618277831802, 80199608722292197]),
        BigInteger([11841292144630778493, 3327346977638983511, 9717945600717261357, 17521983842610332579, 17551948704477232338, 38629783630845674]),
        BigInteger([13053061452543995362, 1573516732916099269, 7734072523617387500, 15879247754331170173, 4985704564687245967, 50017930811101333]),
        BigInteger([5968032006797444201, 4356970211500493920, 14567417169078198555, 5589343791883175912, 13942140483976875481, 77532733089031308]),
        BigInteger([1520419837524800053, 4505352024184739223, 11163238693972155221, 14849828818714008227, 2337273235727598583, 119043856166660542]),
        BigInteger([17856003618470058414, 17971666744702441889, 17477339519519243281, 1190648955970084811, 8535065801186549950, 88202116378298476]),
        BigInteger([3613548573764529932, 17312871435250531950, 14583594317345501622, 15714261421593485983, 3619881023171270578, 102701416776282826]),
        BigInteger([2906047431783924967, 3246340428391821303, 9728912894597247012, 14160632400906136931, 17644388830484393402, 38484906188769515]),
        BigInteger([17914929175761583691, 10159064175292721773, 17508169904588375801, 231704298035432102, 16645242204034044467, 72146101474673684]),
        BigInteger([3958533722314001392, 10367882774307637822, 12047882728743649441, 405962725023303708, 2571821397758431564, 49080211914402704]),
        BigInteger([590761718662001177, 7169147383307934023, 3111750625130109315, 3509576743298836380, 9392819985844593396, 2903460257980110]),
        BigInteger([10894310132299876932, 7222012151301227611, 7759303741675735532, 18168258420254565546, 1036933145679827201, 84980589105836606]),
        BigInteger([6223094491993811174, 10029382893184057307, 1782107595179881914, 4749850940526401858, 17559628252757726982, 20954496417456641]),
        BigInteger([9716738584996322209, 2920283958606294680, 5037648889630024082, 10095866865449441757, 15798169953625136889, 75996191339425194]),
        BigInteger([4358863173918119755, 15055260416528219604, 14884179962427098601, 16487815824678336974, 9163017981789726811, 95112432538757947]),
        BigInteger([2011658397676046806, 15717875473168825780, 280862458342309401, 2045794038349364078, 15857429573515749114, 147064043817050]),
        BigInteger([16760038912651623103, 790302385909476981, 6794528760947774914, 1897898981979607280, 9930859122245863541, 62625600058823796]),
        BigInteger([6859520296144159969, 909173910965215762, 15967585281779797458, 10442593522875662584, 7918433044340205291, 104628130661177253]),
        BigInteger([7212508657599325242, 1584945223518302852, 9963925208591887870, 11598974258151462965, 3264112703271587886, 20341848365764212]),
        BigInteger([2142683228075401710, 5947621269725846341, 5696548463751032717, 13780290877820379740, 10702692037250602632, 33891554982109078]),
        BigInteger([10965161018967488287, 18251363109856037426, 7036083669251591763, 16109345360066746489, 4679973768683352764, 96952949334633821]),
    ];
    #[rustfmt::skip]
    const TWO_ADICITY: u32 = 46u32;
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        2022196864061697551u64,
        17419102863309525423u64,
        8564289679875062096u64,
        17152078065055548215u64,
        17966377291017729567u64,
        68610905582439508u64,
    ]);
}

impl FieldParameters for FqParameters {
    #[rustfmt::skip]
    const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    /// GENERATOR = -5
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xfc0b8000000002fa,
        0x97d39cf6e000018b,
        0x2072420fbfa05044,
        0xcbbcbd50d97c3802,
        0xbaf1ec35813f9eb,
        0x9974a2c0945ad2,
    ]);
    #[rustfmt::skip]
    const INV: u64 = 9586122913090633727u64;
    /// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x8508c00000000001,
        0x170b5d4430000000,
        0x1ef3622fba094800,
        0x1a22d9f300f5138f,
        0xc63b05c06ca1493b,
        0x1ae3a4617c510ea,
    ]);
    #[rustfmt::skip]
    const MODULUS_BITS: u32 = 377;
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x4284600000000000,
        0xb85aea218000000,
        0x8f79b117dd04a400,
        0x8d116cf9807a89c7,
        0x631d82e03650a49d,
        0xd71d230be28875,
    ]);
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        202099033278250856u64,
        5854854902718660529u64,
        11492539364873682930u64,
        8885205928937022213u64,
        5545221690922665192u64,
        39800542322357402u64,
    ]);
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0xb786686c9400cd22,
        0x329fcaab00431b1,
        0x22a5f11162d6b46d,
        0xbfdf7d03827dc3ac,
        0x837e92f041790bf9,
        0x6dfccb1e914b88,
    ]);
    #[rustfmt::skip]
    const REPR_SHAVE_BITS: u32 = 7;
    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    /// T = (MODULUS - 1) // 2^S =
    /// 3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0x7510c00000021423,
        0x88bee82520005c2d,
        0x67cc03d44e3c7bcd,
        0x1701b28524ec688b,
        0xe9185f1443ab18ec,
        0x6b8,
    ]);
    /// (T - 1) // 2 =
    /// 1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xba88600000010a11,
        0xc45f741290002e16,
        0xb3e601ea271e3de6,
        0xb80d94292763445,
        0x748c2f8a21d58c76,
        0x35c,
    ]);
}

impl PoseidonDefaultParameters for FqParameters {
    const PARAMS_OPT_FOR_CONSTRAINTS: [PoseidonDefaultParametersEntry; 7] = [
        PoseidonDefaultParametersEntry::new(2, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(3, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(4, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(5, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(6, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(7, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(8, 5, 8, 57, 0),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_fields::{FftField, Field, PrimeField};

    #[test]
    fn test_powers_of_root_of_unity() {
        let two = Fq::from(2u8);

        // Compute the expected powers of root of unity.
        let root_of_unity = Fq::two_adic_root_of_unity();
        let powers = (0..FqParameters::TWO_ADICITY - 1)
            .map(|i| root_of_unity.pow(two.pow(Fq::from(i as u64).to_bigint()).to_bigint()))
            .collect::<Vec<_>>();
        assert_eq!(powers[0], Fq::two_adic_root_of_unity());

        // Ensure the correct number of powers of root of unity are present.
        assert_eq!(FqParameters::POWERS_OF_ROOTS_OF_UNITY.len() as u64, (FqParameters::TWO_ADICITY - 1) as u64);
        assert_eq!(FqParameters::POWERS_OF_ROOTS_OF_UNITY.len(), powers.len());

        // Ensure the expected and candidate powers match.
        for (expected, candidate) in powers.iter().zip(FqParameters::POWERS_OF_ROOTS_OF_UNITY.iter()) {
            // println!("BigInteger({:?}),", expected.0.0);
            println!("{:?} =?= {:?}", expected.0, candidate);
            assert_eq!(&expected.0, candidate);
        }
    }

    #[test]
    fn test_two_adic_root_of_unity() {
        let expected = Fq::multiplicative_generator().pow(FqParameters::T);
        assert_eq!(expected, Fq::two_adic_root_of_unity());
    }
}
