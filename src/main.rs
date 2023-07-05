use std::env;
use std::fmt::{
    Display, 
    Debug,
    Formatter,
    Result as FmtResult
};

fn main() {
    let args = env::args();

    dbg!(&args);

    let args_u64: Vec<u64> = args.skip(1).map(|x| x.parse::<u64>().unwrap()).collect();

    dbg!(&args_u64);

    let my_ec = EllipticCurve::new(args_u64[0], args_u64[1], args_u64[2]);

    dbg!(&my_ec);

    let point_p = EllipticCurvePoint::new(1, 2, &my_ec);
    let point_q = EllipticCurvePoint::new(3, 4, &my_ec);

    dbg!(&point_p);
    dbg!(&point_q);
}

struct EllipticCurve {
    a: u64,
    b: u64,
    p: u64
}

impl EllipticCurve {
    fn new(a: u64, b: u64, p: u64) -> Self {
        Self { a, b, p }
    }
}

impl Display for EllipticCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "EC(a={}, b={}, p={})", self.a, self.b, self.p)
    }
}

impl Debug for EllipticCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "EC(a={}, b={}, p={})", self.a, self.b, self.p)
    }
}

struct EllipticCurvePoint<'ec> {
    x: u64,
    y: u64,
    ec: &'ec EllipticCurve,
}

impl<'ec> EllipticCurvePoint<'ec> {
    fn new(x: u64, y: u64, ec: &'ec EllipticCurve) -> Self {
        Self { x, y, ec }
    }
}

impl<'ec> Display for EllipticCurvePoint<'ec> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ECPoint(x={}, y={}, ec={})", self.x, self.y, self.ec)
    }
}

impl<'ec> Debug for EllipticCurvePoint<'ec> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ECPoint(x={}, y={}, ec={})", self.x, self.y, self.ec)
    }
}

