use super::imagi;
use super::imagj;
use super::imagk;
use super::real;

use std::ops::Mul;
impl Mul<imagi::ImagI> for real::Real {
    type Output = imagi::ImagI;
    fn mul(self, other: imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for real::Real {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for &real::Real {
    type Output = imagi::ImagI;
    fn mul(self, other: imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for &real::Real {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagi::ImagI) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for real::Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for real::Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for &real::Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for &real::Real {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagj::ImagJ) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for real::Real {
    type Output = imagk::ImagK;
    fn mul(self, other: imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for real::Real {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for &real::Real {
    type Output = imagk::ImagK;
    fn mul(self, other: imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for &real::Real {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagk::ImagK) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for imagi::ImagI {
    type Output = imagi::ImagI;
    fn mul(self, other: real::Real) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for imagi::ImagI {
    type Output = imagi::ImagI;
    fn mul(self, other: &real::Real) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for &imagi::ImagI {
    type Output = imagi::ImagI;
    fn mul(self, other: real::Real) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for &imagi::ImagI {
    type Output = imagi::ImagI;
    fn mul(self, other: &real::Real) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for imagi::ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for imagi::ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for &imagi::ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for &imagi::ImagI {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagj::ImagJ) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for imagi::ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for imagi::ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for &imagi::ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for &imagi::ImagI {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagk::ImagK) -> imagj::ImagJ {
        imagj::ImagJ {
            value: -self.value * other.value,
        }
    }
}

impl Mul<real::Real> for imagj::ImagJ {
    type Output = imagj::ImagJ;
    fn mul(self, other: real::Real) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for imagj::ImagJ {
    type Output = imagj::ImagJ;
    fn mul(self, other: &real::Real) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for &imagj::ImagJ {
    type Output = imagj::ImagJ;
    fn mul(self, other: real::Real) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for &imagj::ImagJ {
    type Output = imagj::ImagJ;
    fn mul(self, other: &real::Real) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for imagj::ImagJ {
    type Output = imagk::ImagK;
    fn mul(self, other: imagi::ImagI) -> imagk::ImagK {
        imagk::ImagK {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for imagj::ImagJ {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagi::ImagI) -> imagk::ImagK {
        imagk::ImagK {
            value: -self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for &imagj::ImagJ {
    type Output = imagk::ImagK;
    fn mul(self, other: imagi::ImagI) -> imagk::ImagK {
        imagk::ImagK {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for &imagj::ImagJ {
    type Output = imagk::ImagK;
    fn mul(self, other: &imagi::ImagI) -> imagk::ImagK {
        imagk::ImagK {
            value: -self.value * other.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_multiply_first() {
        {
            let a = real::Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = real::Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = real::Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = real::Real::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
    }

    #[test]
    fn real_multiply_second() {
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}", z), "+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}", z), "+6j");
        }
    }

    #[test]
    fn real_multiply_third() {
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}", z), "+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}", z), "+6k");
        }
    }

    #[test]
    fn first_multiply_real() {
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = i * r;
            assert_eq!(format!("{}", z), "+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = i * &r;
            assert_eq!(format!("{}", z), "+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = &i * r;
            assert_eq!(format!("{}", z), "+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = &i * &r;
            assert_eq!(format!("{}", z), "+6i");
        }
    }

    #[test]
    fn first_multiply_second() {
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
    }

    #[test]
    fn first_multiply_third() {
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "-6j");
        }
        {
            let a = imagi::ImagI::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "-6j");
        }
    }

    #[test]
    fn second_multiply_real() {
        {
            let a = imagj::ImagJ::new(2.0);
            let b = real::Real::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = real::Real::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = real::Real::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = real::Real::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6j");
        }
    }

    #[test]
    fn second_multiply_first() {
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "-6k");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "-6k");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "-6k");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "-6k");
        }
    }
}
