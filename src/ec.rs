
use std::fmt::{
    Display, 
    Debug,
    Formatter,
    Result as FmtResult
};

#[derive(PartialEq)]
pub struct EllipticCurve {
    pub a: i64,
    pub b: i64,
    pub p: i64
}

impl EllipticCurve {
    pub fn new(a: i64, b: i64, p: i64) -> Self {
        Self { a, b, p }
    }

    pub fn modular_sqrt(&self, a: i64) -> i64 {
        let p = self.p;

        if self.legendre_symbol(a) != 1 { return 0; }
        else if a == 0 { return 0; }
        else if p == 2 { return p; }
        else if p.rem_euclid(4) == 3 { return self.pow(a, (p+1)/4); }

        let mut s = p - 1;
        let mut e = 0;

        while s.rem_euclid(2) == 0 {
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

            for val in 0..r {
                if t == 1 {
                    m = val;
                    break;
                }
                t = self.pow(t, 2);
            }

            if m == 0 { return x; }

            let gs = self.pow(g, self.pow(2, r - m - 1));
            g = (gs * gs).rem_euclid(p);
            x = (x * gs).rem_euclid(p);
            b = (b * g).rem_euclid(p);
            r = m;
        }
    }

    fn legendre_symbol(&self, a: i64) -> i8 {
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

    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }
        else {
            let (g, y, x) = EllipticCurve::egcd(b % a, a);
            return (g, x - ((b/ a) * y), y);
        }
    }

    pub fn modinv(&self, a: i64) -> Option<i64> {
        
        let (g, x, _y) = EllipticCurve::egcd(a, self.p);

        if g != 1 {
            println!("Point x is not on the curve, please select another");
            panic!();
        }
        else {
            return Some(x % self.p);
        }
    }

    pub fn pow(&self, mut base: i64, mut exp: i64) -> i64 {
        let modulus = self.p;

        if modulus == 1 { return 0 }
        let mut result = 1;
        
        base = base % modulus;
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            exp = exp >> 1;
            base = (base * base) % modulus
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
