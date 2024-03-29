pub struct Real {
    pub value: f64,
}

impl Real {
    pub fn new(x: f64) -> Real {
        Real { value: x }
    }
}

use std::ops::Add;
impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<Real> for &Real {
    type Output = Real;
    fn add(self, other: Real) -> Real {
        Real {
            value: self.value + other.value,
        }
    }
}

impl Add<&Real> for Real {
    type Output = Real;
    fn add(self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}
impl Add for &Real {
    type Output = Real;
    fn add(self, other: Self) -> Real {
        Real {
            value: self.value + other.value,
        }
    }
}

use std::ops::Sub;
impl Sub for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<&Real> for Real {
    type Output = Real;
    fn sub(self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}
impl Sub<Real> for &Real {
    type Output = Real;
    fn sub(self, other: Real) -> Real {
        Real {
            value: self.value - other.value,
        }
    }
}

impl Sub for &Real {
    type Output = Real;
    fn sub(self, other: Self) -> Real {
        Real {
            value: self.value - other.value,
        }
    }
}

use super::imagi;
use super::imagj;
use super::imagk;
use std::ops::Mul;
impl Mul for Real {
    type Output = Real;
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul<Real> for &Real {
    type Output = Real;
    fn mul(self, other: Real) -> Real {
        Real {
            value: self.value * other.value,
        }
    }
}

impl Mul<&Real> for Real {
    type Output = Real;
    fn mul(self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul for &Real {
    type Output = Real;
    fn mul(self, other: Self) -> Real {
        Real {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for Real {
    type Output = imagi::ImagI;
    fn mul(self, other: imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for Real {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for &Real {
    type Output = imagi::ImagI;
    fn mul(self, other: imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for &Real {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for &Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for &Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for Real {
    type Output = imagk::ImagK;
    fn mul(self, other: imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for Real {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for &Real {
    type Output = imagk::ImagK;
    fn mul(self, other: imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for &Real {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

use std::ops::Div;
impl Div for Real {
    type Output = Real;
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div<Real> for &Real {
    type Output = Real;
    fn div(self, other: Real) -> Real {
        Real {
            value: self.value / other.value,
        }
    }
}

impl Div<&Real> for Real {
    type Output = Self;
    fn div(self, other: &Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div for &Real {
    type Output = Real;
    fn div(self, other: Self) -> Real {
        Real {
            value: self.value / other.value,
        }
    }
}

use std::fmt;
impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_real_works() {
        let r = Real::new(1.0);
        assert_eq!(r.value, 1.0);
    }
    #[test]
    fn can_add_two_reals() {
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = r1 + r2;
            assert_eq!(r3.value, 3.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = &r1 + r2;
            assert_eq!(r3.value, 3.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = r1 + &r2;
            assert_eq!(r3.value, 3.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = &r1 + &r2;
            assert_eq!(r3.value, 3.0);
        }
    }
    #[test]
    fn can_subtract_two_reals() {
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = r1 - r2;
            assert_eq!(r3.value, -1.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = &r1 - r2;
            assert_eq!(r3.value, -1.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = r1 - &r2;
            assert_eq!(r3.value, -1.0);
        }
        {
            let r1 = Real::new(1.0);
            let r2 = Real::new(2.0);
            let r3 = &r1 - &r2;
            assert_eq!(r3.value, -1.0);
        }
    }
    #[test]
    fn can_multiply_two_reals() {
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(3.0);
            let r3 = r1 * r2;
            assert_eq!(r3.value, 6.0);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(3.0);
            let r3 = &r1 * r2;
            assert_eq!(r3.value, 6.0);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(3.0);
            let r3 = r1 * &r2;
            assert_eq!(r3.value, 6.0);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(3.0);
            let r3 = &r1 * &r2;
            assert_eq!(r3.value, 6.0);
        }
    }
    #[test]
    fn can_divide_two_reals() {
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(5.0);
            let r3 = r1 / r2;
            assert_eq!(r3.value, 0.4);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(5.0);
            let r3 = &r1 / r2;
            assert_eq!(r3.value, 0.4);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(5.0);
            let r3 = r1 / &r2;
            assert_eq!(r3.value, 0.4);
        }
        {
            let r1 = Real::new(2.0);
            let r2 = Real::new(5.0);
            let r3 = &r1 / &r2;
            assert_eq!(r3.value, 0.4);
        }
    }
    #[test]
    fn can_format() {
        let r = Real::new(3.0);
        let s = format!("{}", r);
        assert_eq!(s, "3");
    }

    #[test]
    fn real_multiply_first() {
        {
            let a = Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
    }

    #[test]
    fn real_multiply_second() {
        {
            let r = Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}", z), "+6j");
        }
    }

    #[test]
    fn real_multiply_third() {
        {
            let r = Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}", z), "+6k");
        }
    }
}
