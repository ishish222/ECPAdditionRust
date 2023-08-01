use std::env;
use crate::ec::EllipticCurve;
use crate::ecpoint::EllipticCurvePoint;

mod ec;
mod ecpoint;

fn main() {
    let args = env::args();

    dbg!(&args);

    let args_i64: Vec<i64> = args.skip(1).map(|x| x.parse::<i64>().unwrap()).collect();

    dbg!(&args_i64);

    let my_ec = EllipticCurve::new(args_i64[0], args_i64[1], args_i64[2]);

    dbg!(&my_ec);

    let point_p = EllipticCurvePoint::new(args_i64[3], args_i64[4], &my_ec);
    let point_q = EllipticCurvePoint::new(args_i64[5], args_i64[6], &my_ec);

    dbg!(&point_p);
    dbg!(&point_q);

    let point_r = point_p + point_q;

    dbg!(&point_r);
}
