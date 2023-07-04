use std::env;

fn main() {
    let mut args = env::args();
    dbg!(&args);

    let args_u64: Vec<u64> = args.map(|x| x.parse::<u64>().unwrap_or(0)).collect();

    dbg!(&args_u64);

    let my_ec = EllipticCurve::new(args_u64[1], args_u64[2], args_u64[3]);

    dbg!(&my_ec);
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