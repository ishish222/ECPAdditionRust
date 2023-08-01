
use crate::{
    ec::EllipticCurve,
    ecpoint::EllipticCurvePoint
};

#[test]
fn ecc_addition_test_0_7_37() {

    let my_ec = EllipticCurve::new(0, 7, 37);

    let point_p = EllipticCurvePoint::new(6, 1, &my_ec);
    let point_q = EllipticCurvePoint::new(8, 1, &my_ec);

    let point_r = point_p + point_q;

    let point_s = EllipticCurvePoint::new(23, 36, &my_ec);

    assert_eq!(point_r, point_s);
}

#[test]
fn ecc_addition_test_2_3_97() {

    let my_ec = EllipticCurve::new(2, 3, 97);

    let point_p = EllipticCurvePoint::new(12, 94, &my_ec);
    let point_q = EllipticCurvePoint::new(17, 87, &my_ec);

    let point_r = point_p + point_q;

    let point_s = EllipticCurvePoint::new(4, 50, &my_ec);

    assert_eq!(point_r, point_s);
}


#[test]
fn ecc_addition_test_0_7_101() {

    let my_ec = EllipticCurve::new(0, 7, 101);

    let point_p = EllipticCurvePoint::new(62, 51, &my_ec);
    let point_q = EllipticCurvePoint::new(75, 56, &my_ec);

    let point_r = point_p + point_q;

    let point_s = EllipticCurvePoint::new(52, 15, &my_ec);
    assert_eq!(point_r, point_s);
}
