pub struct ImagI {
    value: f64,
}

impl ImagI {
    fn new(x: f64) -> ImagI {
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

use std::ops::Mul;
impl Mul for ImagI {
    type Output = ImagI;
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul<ImagI> for &ImagI {
    type Output = ImagI;
    fn mul(self, other: ImagI) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

impl Mul<&ImagI> for ImagI {
    type Output = ImagI;
    fn mul(self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul for &ImagI {
    type Output = ImagI;
    fn mul(self, other: Self) -> ImagI {
        ImagI {
            value: self.value * other.value,
        }
    }
}

use std::ops::Div;
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
}

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
    fn new_ImagI_works() {
        let r = ImagI::new(1.0);
        assert_eq!(r.value, 1.0);
    }
    #[test]
    fn can_add_two_ImagIs() {
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
    fn can_subtract_two_ImagIs() {
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
    fn can_multiply_two_ImagIs() {
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = &i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagI::new(2.0);
            let i2 = ImagI::new(3.0);
            let i3 = &i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
    }
    #[test]
    fn can_divide_two_ImagIs() {
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
    }
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
}
