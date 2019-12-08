pub struct Real {
    value: f64,
}

impl Real {
    fn new(x: f64) -> Real {
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
    fn add(self, other: &Real) -> Real {
        Real {
            value: self.value + other.value,
        }
    }
}
impl Add for &Real {
    type Output = Real;
    fn add(self, other: &Real) -> Real {
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
    fn sub(self, other: &Real) -> Self {
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
    fn mul(self, other: &Real) -> Real {
        Real {
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
}