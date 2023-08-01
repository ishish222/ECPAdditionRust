use crate::ec::EllipticCurve;
use std::fmt::{
    Display, 
    Debug,
    Formatter,
    Result as FmtResult
};
use std::ops::Add;
use num::Integer;

#[derive(PartialEq)]
pub struct EllipticCurvePoint<'ec> {
    x: i64,
    y: i64,
    ec: &'ec EllipticCurve,
}

impl<'ec> EllipticCurvePoint<'ec> {
    pub fn new(x: i64, y: i64, ec: &'ec EllipticCurve) -> Self {
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
        let a = self.ec.a;
        let b = self.ec.b;
        let p = self.ec.p;
 
        let z=(self.ec.pow(self.x, 3) + a * self.x + b) % p;
        let y1=self.ec.modular_sqrt(z);

        let z=(self.ec.pow(rhs.x, 3) + a * rhs.x + b) % p;
        let y2=self.ec.modular_sqrt(z);

        let diff = (rhs.x-self.x) % p;
        let modinv_ = self.ec.modinv(diff);
        
        match modinv_ {
            Some(modinv) => {
                let s=(y2-y1) * modinv;

                let mut x3 = (self.ec.pow(s, 2)-rhs.x-self.x) % p;
                let mut y3 = (s*(rhs.x-x3)-y2) % p;
        
                x3 = x3.mod_floor(&p);
                y3 = y3.mod_floor(&p);
        
                Self {
                    x: x3,
                    y: y3,
                    ec: self.ec,
                }
            },
            None => {
                println!("Mod inverse does not exist for given parameters");
                panic!();
            }
            
        }

    }
}
