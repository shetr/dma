
use std::cmp::PartialEq;
use std::ops::Rem;
use std::default::Default;

/// Returns true if `a` divides `b`. Otherwise returns false.
///
/// Let's say that `a` divides `b` if there exists `k` such that `b = k * a`.
/// 
/// # Examples
/// 
/// ```
/// use dma::*;
/// 
/// assert_eq!(divides(0, 0), true);
/// assert_eq!(divides(5, 10), true);
/// assert_eq!(divides(5, 7), false);
/// assert_eq!(divides(1, -1), true);
/// assert_eq!(divides(5, -10), true);
/// assert_eq!(divides(5, -7), false);
/// ```
pub fn divides<T>(a: T, b: T) -> bool
    where T: PartialEq + Rem<Output = T> + Default
{
    if a != Default::default() { b % a == Default::default() } else { true }
}

/// Returns true if `b` is divisible by `a`. Otherwise returns false.
/// `b` is divisible by `a` if `a` divides `b`. 
/// Go to [divides] for further information.
pub fn is_divisible_by(a: i32, b: i32) -> bool {
    divides(b, a)
}

pub fn gdc(a: i32, b: i32) -> i32 {
    0
}

fn gdc_euclid(a: i32, b: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn divides_0_0() {
        assert_eq!(divides(0, 0), true);
    }
    #[test]
    fn divides_0_1() {
        assert_eq!(divides(0, 1), true);
    }
    #[test]
    fn divides_1_0() {
        assert_eq!(divides(1, 0), true);
    }
    #[test]
    fn divides_1_1() {
        assert_eq!(divides(1, 1), true);
    }
    #[test]
    fn divides_1_2() {
        assert_eq!(divides(1, 2), true);
    }
    #[test]
    fn divides_2_6() {
        assert_eq!(divides(2, 6), true);
    }
    #[test]
    fn divides_2_5() {
        assert_eq!(divides(2, 5), false);
    }
    #[test]
    fn divides_0_m1() {
        assert_eq!(divides(0, -1), true);
    }
    #[test]
    fn divides_m1_0() {
        assert_eq!(divides(-1, 0), true);
    }
    #[test]
    fn divides_m1_m1() {
        assert_eq!(divides(-1, -1), true);
    }
    #[test]
    fn divides_m2_6() {
        assert_eq!(divides(-2, 6), true);
    }
    #[test]
    fn divides_2_m6() {
        assert_eq!(divides(2, -6), true);
    }
    #[test]
    fn divides_m2_m6() {
        assert_eq!(divides(-2, -6), true);
    }
    #[test]
    fn divides_m2_5() {
        assert_eq!(divides(-2, 5), false);
    }
    #[test]
    fn divides_2_m5() {
        assert_eq!(divides(2, -5), false);
    }
    #[test]
    fn divides_m2_m5() {
        assert_eq!(divides(-2, -5), false);
    }


    fn test_gdc(a: i32, b: i32, res: i32) {
        assert_eq!(gdc(a, b), res);
        assert_eq!(gdc(b, a), res);
    }

    #[test]
    fn gdc_0() {
        test_gdc(0, 0, 0);
    }
    #[test]
    fn gdc_1() {
        test_gdc(1, 0, 1);
        test_gdc(1, 1, 1);
        test_gdc(1, 10, 1);
        test_gdc(2, 3, 1);
        test_gdc(2, 5, 1);
        test_gdc(18, 19, 1);
    }
    #[test]
    fn gdc_2() {
        test_gdc(2, 6, 2);
        test_gdc(6, 10, 2);
        test_gdc(10, 14, 2);
    }
    #[test]
    fn gdc_3() {
        test_gdc(3, 6, 3);
        test_gdc(6, 9, 3);
        test_gdc(9, 12, 3);
    }
    #[test]
    fn gdc_6() {
        test_gdc(6, 12, 6);
        test_gdc(12, 18, 6);
    }
}