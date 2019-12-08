pub struct ImagJ {
    value: f64,
}

impl ImagJ {
    fn new(x: f64) -> ImagJ {
        ImagJ { value: x }
    }
}

use std::ops::Add;
impl Add for ImagJ {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<ImagJ> for &ImagJ {
    type Output = ImagJ;
    fn add(self, other: ImagJ) -> ImagJ {
        ImagJ {
            value: self.value + other.value,
        }
    }
}

impl Add<&ImagJ> for ImagJ {
    type Output = ImagJ;
    fn add(self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}
impl Add for &ImagJ {
    type Output = ImagJ;
    fn add(self, other: Self) -> ImagJ {
        ImagJ {
            value: self.value + other.value,
        }
    }
}

use std::ops::Sub;
impl Sub for ImagJ {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<&ImagJ> for ImagJ {
    type Output = ImagJ;
    fn sub(self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}
impl Sub<ImagJ> for &ImagJ {
    type Output = ImagJ;
    fn sub(self, other: ImagJ) -> ImagJ {
        ImagJ {
            value: self.value - other.value,
        }
    }
}

impl Sub for &ImagJ {
    type Output = ImagJ;
    fn sub(self, other: Self) -> ImagJ {
        ImagJ {
            value: self.value - other.value,
        }
    }
}

use std::ops::Mul;
impl Mul for ImagJ {
    type Output = ImagJ;
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul<ImagJ> for &ImagJ {
    type Output = ImagJ;
    fn mul(self, other: ImagJ) -> ImagJ {
        ImagJ {
            value: self.value * other.value,
        }
    }
}

impl Mul<&ImagJ> for ImagJ {
    type Output = ImagJ;
    fn mul(self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul for &ImagJ {
    type Output = ImagJ;
    fn mul(self, other: Self) -> ImagJ {
        ImagJ {
            value: self.value * other.value,
        }
    }
}

use std::ops::Div;
impl Div for ImagJ {
    type Output = ImagJ;
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div<ImagJ> for &ImagJ {
    type Output = ImagJ;
    fn div(self, other: ImagJ) -> ImagJ {
        ImagJ {
            value: self.value / other.value,
        }
    }
}

impl Div<&ImagJ> for ImagJ {
    type Output = Self;
    fn div(self, other: &Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div for &ImagJ {
    type Output = ImagJ;
    fn div(self, other: Self) -> ImagJ {
        ImagJ {
            value: self.value / other.value,
        }
    }
}

use std::fmt;
impl fmt::Display for ImagJ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value > 0.0 {
            write!(f, "+{}j", self.value)
        } else {
            write!(f, "{}j", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_ImagJ_works() {
        let r = ImagJ::new(1.0);
        assert_eq!(r.value, 1.0);
    }
    #[test]
    fn can_add_two_ImagJs() {
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = &i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = &i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
    }
    #[test]
    fn can_subtract_two_ImagJs() {
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = &i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagJ::new(1.0);
            let i2 = ImagJ::new(2.0);
            let i3 = &i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
    }
    #[test]
    fn can_multiply_two_ImagJs() {
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(3.0);
            let i3 = i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(3.0);
            let i3 = &i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(3.0);
            let i3 = i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(3.0);
            let i3 = &i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
    }
    #[test]
    fn can_divide_two_ImagJs() {
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(5.0);
            let i3 = i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(5.0);
            let i3 = &i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(5.0);
            let i3 = i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagJ::new(2.0);
            let i2 = ImagJ::new(5.0);
            let i3 = &i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
    }
    #[test]
    fn can_format() {
        {
            let r = ImagJ::new(3.0);
            let s = format!("{}", r);
            assert_eq!(s, "+3j");
        }
        {
            let r = ImagJ::new(-3.0);
            let s = format!("{}", r);
            assert_eq!(s, "-3j");
        }
    }
}
