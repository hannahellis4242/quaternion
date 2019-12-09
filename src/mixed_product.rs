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

impl Mul<imagk::ImagK> for imagj::ImagJ {
    type Output = imagi::ImagI;
    fn mul(self, other: imagk::ImagK) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for imagj::ImagJ {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagk::ImagK) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagk::ImagK> for &imagj::ImagJ {
    type Output = imagi::ImagI;
    fn mul(self, other: imagk::ImagK) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagk::ImagK> for &imagj::ImagJ {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagk::ImagK) -> imagi::ImagI {
        imagi::ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for imagk::ImagK {
    type Output = imagk::ImagK;
    fn mul(self, other: real::Real) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for imagk::ImagK {
    type Output = imagk::ImagK;
    fn mul(self, other: &real::Real) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<real::Real> for &imagk::ImagK {
    type Output = imagk::ImagK;
    fn mul(self, other: real::Real) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&real::Real> for &imagk::ImagK {
    type Output = imagk::ImagK;
    fn mul(self, other: &real::Real) -> imagk::ImagK {
        imagk::ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for imagk::ImagK {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagi::ImagI) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for imagk::ImagK {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagi::ImagI) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagi::ImagI> for &imagk::ImagK {
    type Output = imagj::ImagJ;
    fn mul(self, other: imagi::ImagI) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&imagi::ImagI> for &imagk::ImagK {
    type Output = imagj::ImagJ;
    fn mul(self, other: &imagi::ImagI) -> imagj::ImagJ {
        imagj::ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for imagk::ImagK {
    type Output = imagi::ImagI;
    fn mul(self, other: imagj::ImagJ) -> imagi::ImagI {
        imagi::ImagI {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for imagk::ImagK {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagj::ImagJ) -> imagi::ImagI {
        imagi::ImagI {
            value: -self.value * other.value,
        }
    }
}

impl Mul<imagj::ImagJ> for &imagk::ImagK {
    type Output = imagi::ImagI;
    fn mul(self, other: imagj::ImagJ) -> imagi::ImagI {
        imagi::ImagI {
            value: -self.value * other.value,
        }
    }
}

impl Mul<&imagj::ImagJ> for &imagk::ImagK {
    type Output = imagi::ImagI;
    fn mul(self, other: &imagj::ImagJ) -> imagi::ImagI {
        imagi::ImagI {
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

    #[test]
    fn second_multiply_third() {
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6i");
        }
        {
            let a = imagj::ImagJ::new(2.0);
            let b = imagk::ImagK::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6i");
        }
    }

    #[test]
    fn third_multiply_real() {
        {
            let a = imagk::ImagK::new(2.0);
            let b = real::Real::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = real::Real::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = real::Real::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6k");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = real::Real::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6k");
        }
    }

    #[test]
    fn third_multiply_first() {
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "+6j");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagi::ImagI::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "+6j");
        }
    }

    #[test]
    fn third_multiply_second() {
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * b;
            assert_eq!(format!("{}", c), "-6i");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = a * &b;
            assert_eq!(format!("{}", c), "-6i");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * b;
            assert_eq!(format!("{}", c), "-6i");
        }
        {
            let a = imagk::ImagK::new(2.0);
            let b = imagj::ImagJ::new(3.0);
            let c = &a * &b;
            assert_eq!(format!("{}", c), "-6i");
        }
    }

    /*#[test]
    fn triple_product() {
        let a = imagi::ImagI::new(1.0);
        let b = imagj::ImagJ::new(1.0);
        let c = imagk::ImagK::new(1.0);
        let d = a * b * c;
        assert_eq!(format!("{}", d), "-1");
    }*/
}
