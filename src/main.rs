use core::panic;
use std::env;
use std::fmt::{
    Display, 
    Debug,
    Formatter,
    Result as FmtResult, Error
};
use std::ops::Add;


fn main() {
    let args = env::args();

    dbg!(&args);

    let args_u64: Vec<u64> = args.skip(1).map(|x| x.parse::<u64>().unwrap()).collect();

    dbg!(&args_u64);

    // add checking that p parameter is prime number
    let my_ec = EllipticCurve::new(args_u64[0], args_u64[1], args_u64[2]);

    dbg!(&my_ec);

    let point_p = EllipticCurvePoint::new(6, 1, &my_ec);
    let point_q = EllipticCurvePoint::new(8, 1, &my_ec);

    dbg!(&point_p);
    dbg!(&point_q);

/* 
    println!("1^2 % p = {}", my_ec.pow(1, 2));
    println!("2^2 % p = {}", my_ec.pow(2, 2));
    println!("3^2 % p = {}", my_ec.pow(3, 2));
    println!("4^2 % p = {}", my_ec.pow(4, 2));

    println!("ls(1) % p = {}", my_ec.legendre_symbol(1));
    println!("ls(2) % p = {}", my_ec.legendre_symbol(2));
    println!("ls(3) % p = {}", my_ec.legendre_symbol(3));
    println!("ls(4) % p = {}", my_ec.legendre_symbol(4));
    println!("ls(5) % p = {}", my_ec.legendre_symbol(5));
    println!("ls(6) % p = {}", my_ec.legendre_symbol(6));
    println!("ls(7) % p = {}", my_ec.legendre_symbol(7));
    println!("ls(8) % p = {}", my_ec.legendre_symbol(8));
    println!("ls(9) % p = {}", my_ec.legendre_symbol(9));
    println!("ls(10) % p = {}", my_ec.legendre_symbol(10));
*/
    let point_r = point_p + point_q;

    dbg!(&point_r);
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

    fn modular_sqrt(&self, a: u64) -> u64 {
        let p = self.p;

        if self.legendre_symbol(a) != 1 { return 0; }
        else if a == 0 { return 0; }
        else if p == 2 { return p; }
        else if p % 4 == 3 { return self.pow(a, (p+1)/4); }

        let mut s = p - 1;
        let mut e = 0;

        while s % 2 == 0 {
            s /= 2;
            e += 1;
        }

        let mut n = 2;
        while self.legendre_symbol(n) != -1 { n += 1; }

        let mut x = self.pow(a, (s + 1) / 2);
        let mut b = self.pow(a, s);
        let mut g = self.pow(n, s);
        let mut r = e;

        loop {
            let mut t = b;
            let mut m = 0;

            for m in 0..r {
                if t == 1 {
                    break;
                }
                t = self.pow(t, 2);
            }

            if m == 0 { return x; }

            let gs = self.pow(g, self.pow(2, r - m - 1));
            g = (gs * gs) % p;
            x = (x * gs) % p;
            b = (b * g) % p;
            r = m;
        }
    }

    fn legendre_symbol(&self, a: u64) -> i8 {
        let p = self.p;
        let ls = self.pow(a, (p - 1) / 2);
        if ls == p-1 {
            -1
        }
        else 
        {
            let ret:Result<i8, _> = ls.try_into();
            match ret {
                Ok(r) => {return r}
                Err(_) => { dbg!("Error while calculating legendre_symbol"); return 0;}
            }
        }
    }

    fn egcd(a: u64, b: u64) -> (u64, u64, u64) {
        if a == 0 {
            return (b, 0, 1);
        }
        else {
            let (g, y, x) = EllipticCurve::egcd(b % a, a);
            return (g, x - (b / a) * y, y);
        }
    }

    fn modinv(&self, a: u64) -> u64 {
        let (g, x, y) = EllipticCurve::egcd(a, self.p);
        if g != 1 {
            println!("Point x is not on the curve, please select another");
            panic!();
        }
        else {
            return x % self.p;
        }
    }

    fn pow(&self, mut base: u64, mut exp: u64) -> u64 {
        let modulus = self.p;

        if modulus == 1 { return 0 }
        let mut result = 1;
        
        base = base % modulus;
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp = exp >> 1;
            base = base * base % modulus
        }
        result
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

impl<'ec> Add for EllipticCurvePoint<'ec> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let grad_y = rhs.y - self.y;
        let grad_x = rhs.x - self.x;
        let grad = grad_y / grad_x;

        let a = self.ec.a;
        let b = self.ec.b;
        let p = self.ec.p;

        let z=(self.ec.pow(self.x, 3) + a * self.x + b) % p;
        let y1=self.ec.modular_sqrt(z);

        let z=(self.ec.pow(rhs.x, 3) + a * rhs.x + b) % p;
        let y2=self.ec.modular_sqrt(z);

        let s=(y2-y1)* self.ec.modinv(self.x-rhs.x);

        let x3 = (self.ec.pow(s, 2)-self.x-rhs.x) % p;
        let y3 = (s*(rhs.x-x3)-y2) % p;

        Self {
            x: x3,
            y: y3,
            ec: self.ec,
        }
    }
}