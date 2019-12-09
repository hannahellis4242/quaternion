pub struct ImagI {
    pub value: f64,
}

impl ImagI {
    pub fn new(x: f64) -> ImagI {
        ImagI { value: x }
    }
}

use std::ops::Add;
impl Add for ImagI {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<ImagI> for &ImagI {
    type Output = ImagI;
    fn add(self, other: ImagI) -> ImagI {
        ImagI {
            value: self.value + other.value,
        }
    }
}

impl Add<&ImagI> for ImagI {
    type Output = ImagI;
    fn add(self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}
impl Add for &ImagI {
    type Output = ImagI;
    fn add(self, other: Self) -> ImagI {
        ImagI {
            value: self.value + other.value,
        }
    }
}

use std::ops::Sub;
impl Sub for ImagI {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<&ImagI> for ImagI {
    type Output = ImagI;
    fn sub(self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<ImagI> for &ImagI {
    type Output = ImagI;
    fn sub(self, other: ImagI) -> ImagI {
        ImagI {
            value: self.value - other.value,
        }
    }
}

impl Sub for &ImagI {
    type Output = ImagI;
    fn sub(self, other: Self) -> ImagI {
        ImagI {
            value: self.value - other.value,
        }
    }
}

use super::imagj;
use super::imagk;
use super::real;
use std::ops::Mul;
impl Mul for ImagI {
    type Output = real::Real;
    fn mul(self, other: Self) -> real::Real {
        real::Real {
            value: -self.value * other.value,
        }
    }
}

impl Mul<ImagI> for &ImagI {
    type Output = real::Real;
    fn mul(self, other: ImagI) -> real::Real {
        real::Real {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&ImagI> for ImagI {
    type Output = real::Real;
    fn mul(self, other: &Self) -> real::Real {
        real::Real {
            value: -self.value * other.value,
        }
    }
}

impl Mul for &ImagI {
    type Output = real::Real;
    fn mul(self, other: Self) -> real::Real {
        real::Real {
            value: -self.value * other.value,
        }
    }
}

impl Mul<real::Real> for ImagI {
    type Output = ImagI;
    fn mul(self, other: real::Real) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for ImagI {
    type Output = ImagI;
    fn mul(self, other: &real::Real) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for &ImagI {
    type Output = ImagI;
    fn mul(self, other: real::Real) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for &ImagI {
    type Output = ImagI;
    fn mul(self, other: &real::Real) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for &ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for &ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for &ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for &ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

/*use std::ops::Div;
impl Div for ImagI {
    type Output = ImagI;
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div<ImagI> for &ImagI {
    type Output = ImagI;
    fn div(self, other: ImagI) -> ImagI {
        ImagI {
            value: self.value / other.value,
        }
    }
}

impl Div<&ImagI> for ImagI {
    type Output = Self;
    fn div(self, other: &Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div for &ImagI {
    type Output = ImagI;
    fn div(self, other: Self) -> ImagI {
        ImagI {
            value: self.value / other.value,
        }
    }
}*/

use std::fmt;
impl fmt::Display for ImagI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value > 0.0 {
            write!(f, "+{}i", self.value)
        } else {
            write!(f, "{}i", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_imag_i_works() {
        let r = ImagI::new(1.0);
        assert_eq!(r.value, 1.0);
    }
    #[test]
    fn can_add_two_imag_is() {
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = &i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = &i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
    }
    #[test]
    fn can_subtract_two_imag_is() {
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = &i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagI::new(1.0);
            let i2 = ImagI::new(2.0);
            let i3 = &i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
    }
    #[test]
    fn can_multiply_two_imag_is() {
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = i1 * i2;
            assert_eq!(i3.value, -6.0);
            assert_eq!(format!("{}", i3), "-6");
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = &i1 * i2;
            assert_eq!(i3.value, -6.0);
            assert_eq!(format!("{}", i3), "-6");
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = i1 * &i2;
            assert_eq!(i3.value, -6.0);
            assert_eq!(format!("{}", i3), "-6");
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = &i1 * &i2;
            assert_eq!(i3.value, -6.0);
            assert_eq!(format!("{}", i3), "-6");
        }
    }
    /*#[test]
    fn can_divide_two_imag_is() {
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(5.0);
            let i3 = i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(5.0);
            let i3 = &i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(5.0);
            let i3 = i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(5.0);
            let i3 = &i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
    }*/
    #[test]
    fn can_format() {
        {
            let r = ImagI::new(3.0);
            let s = format!("{}", r);
            assert_eq!(s, "+3i");
        }
        {
            let r = ImagI::new(-3.0);
            let s = format!("{}", r);
            assert_eq!(s, "-3i");
        }
    }

    #[test]
    fn first_multiply_real() {
        {
            let a = ImagI::new(3.0);
            let b = real::Real::new(2.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = ImagI::new(3.0);
            let b = real::Real::new(2.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = ImagI::new(3.0);
            let b = real::Real::new(2.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = ImagI::new(3.0);
            let b = real::Real::new(2.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
    }

    #[test]
    fn first_multiply_second() {
        {
            let a = ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
    }

    #[test]
    fn first_multiply_third() {
        {
            let a = ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "-6j");
        }
    }
}
