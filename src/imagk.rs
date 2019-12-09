pub struct ImagK {
    pub value: f64,
}

impl ImagK {
    pub fn new(x: f64) -> ImagK {
        ImagK { value: x }
    }
}

use std::ops::Add;
impl Add for ImagK {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<ImagK> for &ImagK {
    type Output = ImagK;
    fn add(self, other: ImagK) -> ImagK {
        ImagK {
            value: self.value + other.value,
        }
    }
}

impl Add<&ImagK> for ImagK {
    type Output = ImagK;
    fn add(self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}
impl Add for &ImagK {
    type Output = ImagK;
    fn add(self, other: Self) -> ImagK {
        ImagK {
            value: self.value + other.value,
        }
    }
}

use std::ops::Sub;
impl Sub for ImagK {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Sub<&ImagK> for ImagK {
    type Output = ImagK;
    fn sub(self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}
impl Sub<ImagK> for &ImagK {
    type Output = ImagK;
    fn sub(self, other: ImagK) -> ImagK {
        ImagK {
            value: self.value - other.value,
        }
    }
}

impl Sub for &ImagK {
    type Output = ImagK;
    fn sub(self, other: Self) -> ImagK {
        ImagK {
            value: self.value - other.value,
        }
    }
}

use std::ops::Mul;
impl Mul for ImagK {
    type Output = ImagK;
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul<ImagK> for &ImagK {
    type Output = ImagK;
    fn mul(self, other: ImagK) -> ImagK {
        ImagK {
            value: self.value * other.value,
        }
    }
}

impl Mul<&ImagK> for ImagK {
    type Output = ImagK;
    fn mul(self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Mul for &ImagK {
    type Output = ImagK;
    fn mul(self, other: Self) -> ImagK {
        ImagK {
            value: self.value * other.value,
        }
    }
}

use std::ops::Div;
impl Div for ImagK {
    type Output = ImagK;
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div<ImagK> for &ImagK {
    type Output = ImagK;
    fn div(self, other: ImagK) -> ImagK {
        ImagK {
            value: self.value / other.value,
        }
    }
}

impl Div<&ImagK> for ImagK {
    type Output = Self;
    fn div(self, other: &Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl Div for &ImagK {
    type Output = ImagK;
    fn div(self, other: Self) -> ImagK {
        ImagK {
            value: self.value / other.value,
        }
    }
}

use std::fmt;
impl fmt::Display for ImagK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value > 0.0 {
            write!(f, "+{}k", self.value)
        } else {
            write!(f, "{}k", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_imag_k_works() {
        let r = ImagK::new(1.0);
        assert_eq!(r.value, 1.0);
    }
    #[test]
    fn can_add_two_imag_ks() {
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = &i1 + i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = &i1 + &i2;
            assert_eq!(i3.value, 3.0);
        }
    }
    #[test]
    fn can_subtract_two_imag_ks() {
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = &i1 - i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
        {
            let i1 = ImagK::new(1.0);
            let i2 = ImagK::new(2.0);
            let i3 = &i1 - &i2;
            assert_eq!(i3.value, -1.0);
        }
    }
    #[test]
    fn can_multiply_two_imag_ks() {
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(3.0);
            let i3 = i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(3.0);
            let i3 = &i1 * i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(3.0);
            let i3 = i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(3.0);
            let i3 = &i1 * &i2;
            assert_eq!(i3.value, 6.0);
        }
    }
    #[test]
    fn can_divide_two_imag_ks() {
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(5.0);
            let i3 = i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(5.0);
            let i3 = &i1 / i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(5.0);
            let i3 = i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
        {
            let i1 = ImagK::new(2.0);
            let i2 = ImagK::new(5.0);
            let i3 = &i1 / &i2;
            assert_eq!(i3.value, 0.4);
        }
    }
    #[test]
    fn can_format() {
        {
            let r = ImagK::new(3.0);
            let s = format!("{}", r);
            assert_eq!(s, "+3k");
        }
        {
            let r = ImagK::new(-3.0);
            let s = format!("{}", r);
            assert_eq!(s, "-3k");
        }
    }
}
