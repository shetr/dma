
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Rem;
use std::ops::Mul;
use std::ops::Div;

use num_traits::Zero;
use num_traits::OptAbs;

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
    where T: PartialEq + Rem<Output = T> + Zero
{
    if a != T::zero() { b % a == T::zero() } else { true }
}

/// Returns true if `a` is divisible by `b`. Otherwise returns false.
/// `a` is divisible by `b` if `b` divides `a`. 
/// Go to [divides] for further information.
pub fn is_divisible_by<T>(a: T, b: T) -> bool
    where T: PartialEq + Rem<Output = T> + Zero
{
    divides(b, a)
}

/// Returns true if `d` is common divisor of `a` and `b`. Otherwise returns false.
pub fn is_common_divisor<T>(d: T, a: T, b: T) -> bool
    where T: PartialEq + Rem<Output = T> + Clone + Zero
{
    divides(d.clone(), a) && divides(d, b)
}

/// Returns true if `d` is common multiple of `a` and `b`. Otherwise returns false.
pub fn is_common_multiple<T>(d: T, a: T, b: T) -> bool
    where T: PartialEq + Rem<Output = T> + Clone + Zero
{
    divides(a, d.clone()) && divides(b, d)
}

/// Computes greatest common divisor of `a` and `b`.
/// 
/// We define the greatest common divisor as the largest element of the set of common divisors if at least one of `a`, `b` is nonzero.
/// Otherwise we define `gcd(0, 0) = 0`.
pub fn gcd<T>(a: T, b: T) -> T 
    where T: PartialOrd + Rem<Output = T> + Clone + Zero + OptAbs
{
    gcd_noabs(a.opt_abs(), b.opt_abs())
}

/// Computes least common multiple of `a` and `b`.
/// 
/// We define the least common multiple as the smallest element of the set of common multiples if both `a`, `b` are nonzero.
/// Otherwise we define `lmc(a, 0) = lmc(0, b) = 0`.
pub fn lcm<T>(mut a: T, mut b: T) -> T 
    where T: PartialOrd + Rem<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Zero + OptAbs
{
    if a == T::zero() || b == T::zero() {
        return T::zero();
    }
    a = a.opt_abs();
    b = b.opt_abs();
    (a.clone() * b.clone()) / gcd_noabs(a, b)
}

/// Computes greatest common divisor of `a` and `b`,
/// where `a` and `b` are not negative. 
fn gcd_noabs<T>(a: T, b: T) -> T 
    where T: PartialOrd + Rem<Output = T> + Clone + Zero
{
    match (a, b) {
        (a, b) if a == T::zero() && b == T::zero() => T::zero(),
        (a, b) if b == T::zero() => a,
        (a, b) if a == T::zero() => b,
        (a, b) if a > b => gcd_euclid(a, b),
        (a, b) if a < b => gcd_euclid(b, a),
        (a, _) => a
    }
}

/// Computes greatest common divisor of `a` and `b`,
/// where `a` and `b` are positive and `a` > `b`.
fn gcd_euclid<T>(mut a: T, mut b: T) -> T 
    where T: PartialEq + Rem<Output = T> + Clone + Zero
{
    loop {
        let r = a % b.clone();
        (a, b) = (b, r.clone());
        if b == T::zero() { break }
    }
    a
}

/*
struct GcdExtendedRes<T>
    where T: PartialOrd + Rem<Output = T> + Clone + Zero + OptAbs
{
    gcd: T,
    x: T,
    y: T
}
fn gcd_extended<T>(mut a: T, mut b: T) -> GcdExtendedRes<T> 
    where T: PartialOrd + Rem<Output = T> + Clone + Zero + OptAbs
{
    let mut b_clone = b.clone();
    loop {
        let r = a % b;
        a = b_clone.clone();
        b = r.clone();
        b_clone = b.clone();
        if b == T::zero() { break }
    }
    a
}

/// Computes greatest common divisor of `a` and `b`,
/// where `a` and `b` are positive and `a` > `b`.
fn gcd_extended_bezout<T>(mut a: T, mut b: T) -> GcdExtendedRes<T> 
    where T: PartialOrd + Rem<Output = T> + Clone + Zero + OptAbs
{
    loop {
        let r = a % b.clone();
        (a, b) = (b, r.clone());
        if b == T::zero() { break }
    }
    a
}*/

#[cfg(test)]
mod tests {

    use super::*;

    fn test_divides<T>(a: T, b: T, res: bool)
        where T: PartialEq + Rem<Output = T> + Clone + Zero
    {
        assert_eq!(divides(a.clone(), b.clone()), res);
        assert_eq!(is_divisible_by(b, a), res);
    }

    #[test]
    fn divides_0_0() {
        test_divides(0, 0, true);
    }
    #[test]
    fn divides_0_1() {
        test_divides(0, 1, true);
    }
    #[test]
    fn divides_1_0() {
        test_divides(1, 0, true);
    }
    #[test]
    fn divides_1_1() {
        test_divides(1, 1, true);
    }
    #[test]
    fn divides_1_2() {
        test_divides(1, 2, true);
    }
    #[test]
    fn divides_2_6() {
        test_divides(2, 6, true);
    }
    #[test]
    fn divides_2_5() {
        test_divides(2, 5, false);
    }
    #[test]
    fn divides_0_m1() {
        test_divides(0, -1, true);
    }
    #[test]
    fn divides_m1_0() {
        test_divides(-1, 0, true);
    }
    #[test]
    fn divides_m1_m1() {
        test_divides(-1, -1, true);
    }
    #[test]
    fn divides_m2_6() {
        test_divides(-2, 6, true);
    }
    #[test]
    fn divides_2_m6() {
        test_divides(2, -6, true);
    }
    #[test]
    fn divides_m2_m6() {
        test_divides(-2, -6, true);
    }
    #[test]
    fn divides_m2_5() {
        test_divides(-2, 5, false);
    }
    #[test]
    fn divides_2_m5() {
        test_divides(2, -5, false);
    }
    #[test]
    fn divides_m2_m5() {
        test_divides(-2, -5, false);
    }


    fn test_gcd(a: i32, b: i32, res: i32) {
        assert_eq!(gcd(a, b), res);
        assert_eq!(gcd(-a, b), res);
        assert_eq!(gcd(a, -b), res);
        assert_eq!(gcd(-a, -b), res);
        assert_eq!(gcd(b, a), res);
        assert_eq!(gcd(-b, a), res);
        assert_eq!(gcd(b, -a), res);
        assert_eq!(gcd(-b, -a), res);
    }

    #[test]
    fn gcd_0() {
        test_gcd(0, 0, 0);
    }
    #[test]
    fn gcd_1() {
        test_gcd(1, 0, 1);
        test_gcd(1, 1, 1);
        test_gcd(1, 10, 1);
        test_gcd(2, 3, 1);
        test_gcd(2, 5, 1);
        test_gcd(18, 19, 1);
    }
    #[test]
    fn gcd_2() {
        test_gcd(2, 6, 2);
        test_gcd(6, 10, 2);
        test_gcd(10, 14, 2);
    }
    #[test]
    fn gcd_3() {
        test_gcd(3, 6, 3);
        test_gcd(6, 9, 3);
        test_gcd(9, 12, 3);
    }
    #[test]
    fn gcd_6() {
        test_gcd(6, 12, 6);
        test_gcd(12, 18, 6);
    }


    fn test_lcm(a: i32, b: i32, res: i32) {
        assert_eq!(lcm(a, b), res);
        assert_eq!(lcm(-a, b), res);
        assert_eq!(lcm(a, -b), res);
        assert_eq!(lcm(-a, -b), res);
        assert_eq!(lcm(b, a), res);
        assert_eq!(lcm(-b, a), res);
        assert_eq!(lcm(b, -a), res);
        assert_eq!(lcm(-b, -a), res);
    }

    #[test]
    fn lcm_0() {
        test_lcm(0, 0, 0);
        test_lcm(1, 0, 0);
        test_lcm(2, 0, 0);
        test_lcm(7, 0, 0);
    }
    #[test]
    fn lcm_1() {
        test_lcm(1, 1, 1);
    }
    #[test]
    fn lcm_2() {
        test_lcm(2, 2, 2);
        test_lcm(1, 2, 2);
    }
    #[test]
    fn lcm_3() {
        test_lcm(3, 3, 3);
        test_lcm(1, 3, 3);
    }
    #[test]
    fn lcm_4() {
        test_lcm(1, 4, 4);
        test_lcm(2, 4, 4);
        test_lcm(4, 4, 4);
    }
    #[test]
    fn lcm_6() {
        test_lcm(1, 6, 6);
        test_lcm(2, 3, 6);
        test_lcm(6, 6, 6);
    }
    #[test]
    fn lcm_10() {
        test_lcm(1, 10, 10);
        test_lcm(2, 5, 10);
        test_lcm(10, 10, 10);
    }
    #[test]
    fn lcm_12() {
        test_lcm(1, 12, 12);
        test_lcm(3, 4, 12);
        test_lcm(12, 12, 12);
    }

}