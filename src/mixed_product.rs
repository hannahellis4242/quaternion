use super::real;
use super::imagi;
use super::imagj;
use super::imagk;

use std::ops::Mul;
impl Mul<imagi::ImagI> for real::Real{
    type Output = imagi::ImagI ;
    fn mul(self,other:imagi::ImagI)->imagi::ImagI{
        imagi::ImagI{value:self.value*other.value}
    }
}

impl Mul<&imagi::ImagI> for real::Real{
    type Output = imagi::ImagI ;
    fn mul(self,other:&imagi::ImagI)->imagi::ImagI{
        imagi::ImagI{value:self.value*other.value}
    }
}

impl Mul<imagi::ImagI> for &real::Real{
    type Output = imagi::ImagI ;
    fn mul(self,other:imagi::ImagI)->imagi::ImagI{
        imagi::ImagI{value:self.value*other.value}
    }
}

impl Mul<&imagi::ImagI> for &real::Real{
    type Output = imagi::ImagI ;
    fn mul(self,other:&imagi::ImagI)->imagi::ImagI{
        imagi::ImagI{value:self.value*other.value}
    }
}

impl Mul<imagj::ImagJ> for real::Real{
    type Output = imagj::ImagJ ;
    fn mul(self,other:imagj::ImagJ)->imagj::ImagJ{
        imagj::ImagJ{value:self.value*other.value}
    }
}

impl Mul<&imagj::ImagJ> for real::Real{
    type Output = imagj::ImagJ ;
    fn mul(self,other:&imagj::ImagJ)->imagj::ImagJ{
        imagj::ImagJ{value:self.value*other.value}
    }
}

impl Mul<imagj::ImagJ> for &real::Real{
    type Output = imagj::ImagJ ;
    fn mul(self,other:imagj::ImagJ)->imagj::ImagJ{
        imagj::ImagJ{value:self.value*other.value}
    }
}

impl Mul<&imagj::ImagJ> for &real::Real{
    type Output = imagj::ImagJ ;
    fn mul(self,other:&imagj::ImagJ)->imagj::ImagJ{
        imagj::ImagJ{value:self.value*other.value}
    }
}

impl Mul<imagk::ImagK> for real::Real{
    type Output = imagk::ImagK ;
    fn mul(self,other:imagk::ImagK)->imagk::ImagK{
        imagk::ImagK{value:self.value*other.value}
    }
}

impl Mul<&imagk::ImagK> for real::Real{
    type Output = imagk::ImagK ;
    fn mul(self,other:&imagk::ImagK)->imagk::ImagK{
        imagk::ImagK{value:self.value*other.value}
    }
}

impl Mul<imagk::ImagK> for &real::Real{
    type Output = imagk::ImagK ;
    fn mul(self,other:imagk::ImagK)->imagk::ImagK{
        imagk::ImagK{value:self.value*other.value}
    }
}

impl Mul<&imagk::ImagK> for &real::Real{
    type Output = imagk::ImagK ;
    fn mul(self,other:&imagk::ImagK)->imagk::ImagK{
        imagk::ImagK{value:self.value*other.value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_multiply_first() {
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}",z),"+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}",z),"+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}",z),"+6i");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagi::ImagI::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}",z),"+6i");
        }
    }

    #[test]
    fn real_multiply_second() {
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}",z),"+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}",z),"+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}",z),"+6j");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagj::ImagJ::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}",z),"+6j");
        }
    }

    #[test]
    fn real_multiply_third() {
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * i;
            assert_eq!(format!("{}",z),"+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = r * &i;
            assert_eq!(format!("{}",z),"+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * i;
            assert_eq!(format!("{}",z),"+6k");
        }
        {
            let r = real::Real::new(2.0);
            let i = imagk::ImagK::new(3.0);
            let z = &r * &i;
            assert_eq!(format!("{}",z),"+6k");
        }
    }
}
