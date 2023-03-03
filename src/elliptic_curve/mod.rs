#![allow(unused)]
pub mod curve;
pub mod point;
pub mod secp256k1;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::finite_fields::macros::felt;
    use num_bigint::BigUint;
    use primitive_types::U256;

    use super::{curve::Curve, point::Point, secp256k1::Secp256k1Point, *};

    #[test]
    fn test_curve() {
        let prime = 223u64;
        let curve = Curve::new(felt!(0, prime), felt!(7, prime));

        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x, y) in valid_points {
            let point = curve.point(felt!(x, prime), felt!(y, prime));
            assert!(point.is_ok());
        }

        for (x, y) in invalid_points {
            let point = curve.point(felt!(x, prime), felt!(y, prime));
            assert!(point.is_err());
        }
    }

    #[test]
    fn test_point_add() {
        let prime = 223u64;

        let curve = Curve::new(felt!(0, prime), felt!(7, prime));

        let pt1 = curve.point(felt!(170, prime), felt!(142, prime)).unwrap();
        let pt2 = curve.point(felt!(60, prime), felt!(139, prime)).unwrap();
        let expected_sum = curve.point(felt!(220, prime), felt!(181, prime)).unwrap();
        assert_eq!(pt1 + pt2, expected_sum);
    }

    #[test]
    fn test_scalar() {
        let scalar_multiples = vec![
            (47, 71),
            (36, 111),
            (15, 137),
            (194, 51),
            (126, 96),
            (139, 137),
            (92, 47),
            (116, 55),
        ];

        let prime = 223u64;
        let curve = Curve::new(felt!(0, prime), felt!(7, prime));
        let generator = curve.point(felt!(47, prime), felt!(71, prime)).unwrap();

        for i in 1..=scalar_multiples.len() as u32 {
            let result = generator.clone() * i;
            let expected = curve
                .point(
                    felt!(scalar_multiples[i as usize - 1].0, prime),
                    felt!(scalar_multiples[i as usize - 1].1, prime),
                )
                .unwrap();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_order() {
        let prime: u64 = 223;
        let a = felt!(0, prime);
        let b = felt!(7, prime);
        let curve = Curve::new(a.clone(), b.clone());

        let mut i: usize = 0;
        let mut point = curve.identity();
        loop {
            let generator = curve.point(felt!(15, prime), felt!(86, prime)).unwrap();
            point = point + generator;
            i += 1;

            if point == curve.identity() {
                break;
            }
        }

        assert_eq!(i, 7);
    }

    #[test]
    fn test_binary_expansion() {
        let prime = 223u64;
        let curve = Curve::new(felt!(0, prime), felt!(7, prime));
        let generator = curve.point(felt!(47, prime), felt!(71, prime)).unwrap();

        for i in 0..10 {
            let coefficient = (i + 1) as u32;
            let naive_multiple = generator.clone().naive_mul(coefficient);
            let binary_expanded = generator.clone().binary_expansion_mul(coefficient);

            assert_eq!(naive_multiple, binary_expanded);
        }
    }

    #[test]
    fn test_secp256k1_values() {
        // The fact that this works means point is on the curve
        let point = Secp256k1Point::g();
        let point: Point = point.clone().into();

        // Compare point values with string representations of the values
        assert_eq!(
            BigUint::from_str(
                "55066263022277343669578718895168534326250603453777594175500187360389116729240"
            )
            .unwrap(),
            point.x.unwrap().inner().to_owned()
        );

        assert_eq!(
            BigUint::from_str(
                "32670510020758816978083085130507043184471273380659243275938904335757337482424"
            )
            .unwrap(),
            point.y.unwrap().inner().to_owned()
        );

        assert_eq!(
            BigUint::from_str(
                "115792089237316195423570985008687907853269984665640564039457584007908834671663"
            )
            .unwrap(),
            point.curve.a.prime().to_owned()
        );
    }

    #[test]
    fn test_secp256k1_scalar() {
        let point = Secp256k1Point::g();
        let identity: Point = (point * Secp256k1Point::order()).into();

        assert_eq!(identity, Secp256k1Point::curve().identity());
    }
}
