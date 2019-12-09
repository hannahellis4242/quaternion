use super::real;
use super::imagi;

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
}
