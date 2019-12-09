use super::imagi;
use super::imagj;
use super::imagk;
use super::real;

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


    /*#[test]
    fn triple_product() {
        let a = imagi::ImagI::new(1.0);
        let b = imagj::ImagJ::new(1.0);
        let c = imagk::ImagK::new(1.0);
        let d = a * b * c;
        assert_eq!(format!("{}", d), "-1");
    }*/
}
