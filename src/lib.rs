mod real;

/*pub struct VecI {
    value: f64,
}

pub struct VecJ {
    value: f64,
}

pub struct VecK {
    value: f64,
}*/

/*pub struct Quarternion {
    real: Real,
    vec_i: VecI,
    vec_j: VecJ,
    vec_k: VecK,
}*/
/*
extern crate num_traits;
use num_traits::Zero;
fn show<T>(x: &T) -> String
where
    T: num_traits::Zero,
    T: PartialEq<T>,
    T: PartialOrd<T>,
    T: fmt::Display,
{
    let z = T::zero();
    if *x == z {
        String::new()
    } else {
        if *x < z {
            format!("{}", x)
        } else {
            format!("+{}", x)
        }
    }
}

impl<T> Quarternion<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Quarternion<T> {
        Quarternion {
            scalar: a,
            vec_i: b,
            vec_j: c,
            vec_k: d,
        }
    }
}

use std::fmt;

impl<T> fmt::Display for Quarternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}i{}j{}k",
            show(&self.scalar),
            show(&self.vec_i),
            show(&self.vec_j),
            show(&self.vec_k)
        )
    }
}
*/
#[cfg(test)]
mod test {
    use super::*;
    /*#[test]
    fn new_works() {
        let q = Quarternion::new(1, 2, 3, 4);
        assert_eq!(q.scalar, 1);
        assert_eq!(q.vec_i, 2);
        assert_eq!(q.vec_j, 3);
        assert_eq!(q.vec_k, 4);
    }

    #[test]
    fn can_print() {
        let q = Quarternion::new(1, 2, 3, 4);
        println!("{}", q);
    }*/
}
