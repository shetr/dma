
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
pub fn divides(a: i64, b: i64) -> bool
{
    if a != 0 { b % a == 0 } else { true }
}

/// Returns true if `a` is divisible by `b`. Otherwise returns false.
/// `a` is divisible by `b` if `b` divides `a`. 
/// Go to [divides] for further information.
pub fn is_divisible_by(a: i64, b: i64) -> bool
{
    divides(b, a)
}

/// Returns true if `d` is common divisor of `a` and `b`. Otherwise returns false.
pub fn is_common_divisor(d: i64, a: i64, b: i64) -> bool
{
    divides(d, a) && divides(d, b)
}

/// Returns true if `d` is common multiple of `a` and `b`. Otherwise returns false.
pub fn is_common_multiple(d: i64, a: i64, b: i64) -> bool
{
    divides(a, d) && divides(b, d)
}

/// Computes greatest common divisor of `a` and `b`.
/// 
/// We define the greatest common divisor as the largest element of the set of common divisors if at least one of `a`, `b` is nonzero.
/// Otherwise we define `gcd(0, 0) = 0`.
pub fn gcd(a: i64, b: i64) -> i64
{
    gcd_noabs(a.abs(), b.abs())
}

/// Computes least common multiple of `a` and `b`.
/// 
/// We define the least common multiple as the smallest element of the set of common multiples if both `a`, `b` are nonzero.
/// Otherwise we define `lmc(a, 0) = lmc(0, b) = 0`.
pub fn lcm(mut a: i64, mut b: i64) -> i64
{
    if a == 0 || b == 0 {
        return 0;
    }
    a = a.abs();
    b = b.abs();
    (a * b) / gcd_noabs(a, b)
}

/// Computes greatest common divisor of `a` and `b`,
/// where `a` and `b` are not negative. 
fn gcd_noabs(a: i64, b: i64) -> i64
{
    match (a, b) {
        (a, b) if a == 0 && b == 0 => 0,
        (a, b) if b == 0 => a,
        (a, b) if a == 0 => b,
        (a, b) if a > b => gcd_euclid(a, b),
        (a, b) if a < b => gcd_euclid(b, a),
        (a, _) => a
    }
}

/// Computes greatest common divisor of `a` and `b`,
/// where `a` and `b` are positive and `a` > `b`.
fn gcd_euclid(mut a: i64, mut b: i64) -> i64 
{
    while b != 0 {
        let r = a % b;
        (a, b) = (b, r);
    }
    a
}

#[derive(PartialEq, Debug)]
pub struct GcdExtendedResult
{
    pub gcd: i64,
    pub x0: i64,
    pub y0: i64,
    pub x1: i64,
    pub y1: i64
}

/// Computes greatest common divisor of `a` and `b`.
/// This is an extended variant which also computes `x0`, `y0`, `x1`, `y1`,
/// satisfying `gcd(a, b) = x0*a + y0*b` and `0 = x1*a + y1*b`.
/// If there are more solutions, only one will be returned.
pub fn gcd_extended(a: i64, b: i64) -> GcdExtendedResult 
{
    let mut res = gcd_extended_noabs(a.abs(), b.abs());
    if a < 0 {
        res.x0 = -res.x0;
        res.x1 = -res.x1;
    }
    if b < 0 {
        res.y0 = -res.y0;
        res.y1 = -res.y1;
    }
    res
}

fn gcd_extended_noabs(a: i64, b: i64) -> GcdExtendedResult 
{
    match (a, b) {
        (a, b) if a == 0 && b == 0 => GcdExtendedResult { gcd: 0, x0: 0, y0: 0, x1: 0, y1: 0 },
        (a, b) if b == 0 => GcdExtendedResult { gcd: a, x0: 1, y0: 0, x1: 0, y1: 0 },
        (a, b) if a == 0 => GcdExtendedResult { gcd: b, x0: 0, y0: 1, x1: 0, y1: 0 },
        (a, b) if a > b => gcd_extended_bezout(a, b),
        (a, b) if a < b => {
            let mut res = gcd_extended_bezout(b, a);
            (res.x0, res.y0) = (res.y0, res.x0);
            (res.x1, res.y1) = (res.y1, res.x1);
            res
        },
        (a, _) => GcdExtendedResult { gcd: a, x0: 1, y0: 0, x1: -1, y1: 1 }
    }
}

fn gcd_extended_bezout(mut a: i64, mut b: i64) -> GcdExtendedResult 
{
    let mut a0 = 1;
    let mut a1 = 0;
    let mut b0 = 0;
    let mut b1 = 1;
    while b != 0 {
        let q = a / b;
        let r = a - b * q;
        (a, b) = (b, r);
        (a0, a1) = (a1, a0 - q * a1);
        (b0, b1) = (b1, b0 - q * b1);
    }
    GcdExtendedResult { gcd: a, x0: a0, y0: b0, x1: a1, y1: b1 }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GcdIteration
{
    pub a: i64,
    pub b: i64
}

/// Iterates through the euclid's alorithm for finging greatest common divisor.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GcdIterator
{
    i: Option<GcdIteration>
}

impl GcdIterator {
    pub fn new(a: i64, b: i64) -> Self {
        GcdIterator { i: Some(GcdIteration{ a, b })}
    }
}

impl Iterator for GcdIterator {
    type Item = GcdIteration;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_iteration = self.i;
        if let Some(i) = self.i {
            self.i = match (i.a, i.b) {
                (a, b) if a < 0 || b < 0 => {
                    Some(GcdIteration { a : a.abs(), b : b.abs() })
                },
                (_, 0) => {
                    None
                },
                (a, b) if a < b => {
                    Some(GcdIteration { a : b, b : a })
                },
                (a, b) => {
                    let r = a % b;
                    Some(GcdIteration { a : b, b : r })
                }
            }
        }
        prev_iteration
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GcdExtendedIteration
{
    pub a: i64,
    pub b: i64,
    pub a0: i64,
    pub a1: i64,
    pub b0: i64,
    pub b1: i64,
    pub q: i64
}

impl GcdExtendedIteration {
    pub fn new(mut a: i64, mut b: i64) -> Self {
        (a, b) = (a.abs(), b.abs());
        if a < b {
            (a, b) = (b, a);
        }
        GcdExtendedIteration { a, b, a0: 1, a1: 0, b0: 0, b1: 1, q: 0 }
    }
}

/// Iterates through extended version of the euclid's alorithm for finging greatest common divisor.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GcdExtendedIterator
{
    i: Option<GcdExtendedIteration>,
}

impl GcdExtendedIterator {
    pub fn new(a: i64, b: i64) -> Self {
        GcdExtendedIterator { i: Some(GcdExtendedIteration::new(a, b))}
    }
}

impl Iterator for GcdExtendedIterator {
    type Item = GcdExtendedIteration;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_iteration = self.i;
        if let Some(i) = self.i {
            self.i = 
            if i.b == 0 {
                None
            } else {
                let (a, b) = (i.a, i.b);
                let q = a / b;
                let r = a - b * q;
                let (a, b) = (b, r);
                let (a0, a1) = (i.a1, i.a0 - q * i.a1);
                let (b0, b1) = (i.b1, i.b0 - q * i.b1);
                let i = GcdExtendedIteration { a, b, a0, a1, b0, b1, q };
                Some(i)
            }
        }
        prev_iteration
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_divides(a: i64, b: i64, res: bool)
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


    fn test_gcd(a: i64, b: i64, res: i64) {
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
    fn gcd_0_0_0() {
        test_gcd(0, 0, 0);
    }
    #[test]
    fn gcd_1_1_0() {
        test_gcd(1, 0, 1);
    }
    #[test]
    fn gcd_1_1_1() {
        test_gcd(1, 1, 1);
    }
    #[test]
    fn gcd_1_1_10() {
        test_gcd(1, 10, 1);
    }
    #[test]
    fn gcd_1_2_3() {
        test_gcd(2, 3, 1);
    }
    #[test]
    fn gcd_1_2_5() {
        test_gcd(2, 5, 1);
    }
    #[test]
    fn gcd_1_18_19() {
        test_gcd(18, 19, 1);
    }
    #[test]
    fn gcd_2_2_6() {
        test_gcd(2, 6, 2);
    }
    #[test]
    fn gcd_2_6_10() {
        test_gcd(6, 10, 2);
    }
    #[test]
    fn gcd_2_10_14() {
        test_gcd(10, 14, 2);
    }
    #[test]
    fn gcd_3_3_6() {
        test_gcd(3, 6, 3);
    }
    #[test]
    fn gcd_3_6_9() {
        test_gcd(6, 9, 3);
    }
    #[test]
    fn gcd_3_9_12() {
        test_gcd(9, 12, 3);
    }
    #[test]
    fn gcd_6_6_12() {
        test_gcd(6, 12, 6);
    }
    #[test]
    fn gcd_6_12_18() {
        test_gcd(12, 18, 6);
    }


    fn test_lcm(a: i64, b: i64, res: i64) {
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
    fn lcm_0_0_0() {
        test_lcm(0, 0, 0);
    }
    #[test]
    fn lcm_0_1_0() {
        test_lcm(1, 0, 0);
    }
    #[test]
    fn lcm_0_2_0() {
        test_lcm(2, 0, 0);
    }
    #[test]
    fn lcm_0_7_0() {
        test_lcm(7, 0, 0);
    }
    #[test]
    fn lcm_1_1_1() {
        test_lcm(1, 1, 1);
    }
    #[test]
    fn lcm_2_2_2() {
        test_lcm(2, 2, 2);
    }
    #[test]
    fn lcm_2_1_2() {
        test_lcm(1, 2, 2);
    }
    #[test]
    fn lcm_3_3_3() {
        test_lcm(3, 3, 3);
    }
    #[test]
    fn lcm_3_1_3() {
        test_lcm(1, 3, 3);
    }
    #[test]
    fn lcm_4_1_4() {
        test_lcm(1, 4, 4);
    }
    #[test]
    fn lcm_4_2_4() {
        test_lcm(2, 4, 4);
    }
    #[test]
    fn lcm_4_4_4() {
        test_lcm(4, 4, 4);
    }
    #[test]
    fn lcm_6_1_6() {
        test_lcm(1, 6, 6);
    }
    #[test]
    fn lcm_6_2_3() {
        test_lcm(2, 3, 6);
    }
    #[test]
    fn lcm_6_6_6() {
        test_lcm(6, 6, 6);
    }
    #[test]
    fn lcm_10_1_10() {
        test_lcm(1, 10, 10);
    }
    #[test]
    fn lcm_10_2_5() {
        test_lcm(2, 5, 10);
    }
    #[test]
    fn lcm_10_10_10() {
        test_lcm(10, 10, 10);
    }
    #[test]
    fn lcm_12_1_12() {
        test_lcm(1, 12, 12);
    }
    #[test]
    fn lcm_12_3_4() {
        test_lcm(3, 4, 12);
    }
    #[test]
    fn lcm_12_12_12() {
        test_lcm(12, 12, 12);
    }

    fn test_gcd_extended(a: i64, b: i64, res: GcdExtendedResult) {
        assert_eq!(gcd_extended( a,  b), res);
        assert_eq!(gcd_extended(-a,  b), GcdExtendedResult { gcd: res.gcd, x0: -res.x0, y0:  res.y0, x1: -res.x1, y1:  res.y1});
        assert_eq!(gcd_extended( a, -b), GcdExtendedResult { gcd: res.gcd, x0:  res.x0, y0: -res.y0, x1:  res.x1, y1: -res.y1});
        assert_eq!(gcd_extended(-a, -b), GcdExtendedResult { gcd: res.gcd, x0: -res.x0, y0: -res.y0, x1: -res.x1, y1: -res.y1});
        assert_eq!(gcd_extended( b,  a), GcdExtendedResult { gcd: res.gcd, x0:  res.y0, y0:  res.x0, x1:  res.y1, y1:  res.x1});
        assert_eq!(gcd_extended(-b,  a), GcdExtendedResult { gcd: res.gcd, x0: -res.y0, y0:  res.x0, x1: -res.y1, y1:  res.x1});
        assert_eq!(gcd_extended( b, -a), GcdExtendedResult { gcd: res.gcd, x0:  res.y0, y0: -res.x0, x1:  res.y1, y1: -res.x1});
        assert_eq!(gcd_extended(-b, -a), GcdExtendedResult { gcd: res.gcd, x0: -res.y0, y0: -res.x0, x1: -res.y1, y1: -res.x1});
    }

    #[test]
    fn gcd_extended_0_0_0() {
        test_gcd_extended(0, 0, GcdExtendedResult { gcd: 0, x0: 0, y0: 0, x1: 0, y1: 0});
    }
    #[test]
    fn gcd_extended_1_1_0() {
        test_gcd_extended(1, 0, GcdExtendedResult { gcd: 1, x0: 1, y0: 0, x1: 0, y1: 0});
    }
    #[test]
    fn gcd_extended_1_1_1() {
        assert_eq!(gcd_extended(1, 1), GcdExtendedResult { gcd: 1, x0: 1, y0: 0, x1: -1, y1: 1});
    }
    #[test]
    fn gcd_extended_1_1_10() {
        test_gcd_extended(1, 10, GcdExtendedResult { gcd: 1, x0: 1, y0: 0, x1: -10, y1: 1});
    }
    #[test]
    fn gcd_extended_1_2_3() {
        test_gcd_extended(2, 3, GcdExtendedResult { gcd: 1, x0: -1, y0: 1, x1: 3, y1: -2});
    }
    #[test]
    fn gcd_extended_1_2_5() {
        test_gcd_extended(2, 5, GcdExtendedResult { gcd: 1, x0: -2, y0: 1, x1: 5, y1: -2});
    }
    #[test]
    fn gcd_extended_1_18_19() {
        test_gcd_extended(18, 19, GcdExtendedResult { gcd: 1, x0: -1, y0: 1, x1: 19, y1: -18});
    }
    #[test]
    fn gcd_extended_2_2_6() {
        test_gcd_extended(2, 6, GcdExtendedResult { gcd: 2, x0: 1, y0: 0, x1: -3, y1: 1});
    }
    #[test]
    fn gcd_extended_2_6_10() {
        test_gcd_extended(6, 10, GcdExtendedResult { gcd: 2, x0: 2, y0: -1, x1: -5, y1: 3});
    }
    #[test]
    fn gcd_extended_2_10_14() {
        test_gcd_extended(10, 14, GcdExtendedResult { gcd: 2, x0: 3, y0: -2, x1: -7, y1: 5});
    }
    #[test]
    fn gcd_extended_3_3_6() {
        test_gcd_extended(3, 6, GcdExtendedResult { gcd: 3, x0: 1, y0: 0, x1: -2, y1: 1});
    }
    #[test]
    fn gcd_extended_3_6_9() {
        test_gcd_extended(6, 9, GcdExtendedResult { gcd: 3, x0: -1, y0: 1, x1: 3, y1: -2});
    }
    #[test]
    fn gcd_extended_3_9_12() {
        test_gcd_extended(9, 12, GcdExtendedResult { gcd: 3, x0: -1, y0: 1, x1: 4, y1: -3});
    }
    #[test]
    fn gcd_extended_6_6_12() {
        test_gcd_extended(6, 12, GcdExtendedResult { gcd: 6, x0: 1, y0: 0, x1: -2, y1: 1});
    }
    #[test]
    fn gcd_extended_6_12_18() {
        test_gcd_extended(12, 18, GcdExtendedResult { gcd: 6, x0: -1, y0: 1, x1: 3, y1: -2});
    }

    #[test]
    fn gcd_iterator_0_0() {
        let mut gcd_it = GcdIterator::new(0, 0);
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: 0, b: 0 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_iterator_1_1() {
        let mut gcd_it = GcdIterator::new(1, 1);
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: 1, b: 1 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: 1, b: 0 }));
        assert_eq!(gcd_it.next(), None);
    }

    #[test]
    fn gcd_iterator_m9_m12() {
        let mut gcd_it = GcdIterator::new(-9, -12);
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: -9, b: -12 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  9, b:  12 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: 12, b:   9 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  9, b:   3 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  3, b:   0 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_iterator_9_m12() {
        let mut gcd_it = GcdIterator::new(9, -12);
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  9, b: -12 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  9, b:  12 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a: 12, b:   9 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  9, b:   3 }));
        assert_eq!(gcd_it.next(), Some(GcdIteration{ a:  3, b:   0 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_extended_iterator_0_0() {
        let mut gcd_it = GcdExtendedIterator::new(0, 0);
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a: 0, b: 0, a0: 1, a1: 0, b0: 0, b1: 1, q: 0 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_extended_iterator_1_1() {
        let mut gcd_it = GcdExtendedIterator::new(1, 1);
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a: 1, b: 1, a0: 1, a1: 0, b0: 0, b1:  1, q: 0 }));
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a: 1, b: 0, a0: 0, a1: 1, b0: 1, b1: -1, q: 1 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_extended_iterator_m9_m12() {
        let mut gcd_it = GcdExtendedIterator::new(-9, -12);
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a: 12, b: 9, a0: 1, a1:  0, b0:  0, b1:  1, q: 0 }));
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a:  9, b: 3, a0: 0, a1:  1, b0:  1, b1: -1, q: 1 }));
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a:  3, b: 0, a0: 1, a1: -3, b0: -1, b1:  4, q: 3 }));
        assert_eq!(gcd_it.next(), None);
    }
    
    #[test]
    fn gcd_extended_iterator_9_m12() {
        let mut gcd_it = GcdExtendedIterator::new(9, -12);
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a: 12, b: 9, a0: 1, a1:  0, b0:  0, b1:  1, q: 0 }));
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a:  9, b: 3, a0: 0, a1:  1, b0:  1, b1: -1, q: 1 }));
        assert_eq!(gcd_it.next(), Some(GcdExtendedIteration{ a:  3, b: 0, a0: 1, a1: -3, b0: -1, b1:  4, q: 3 }));
        assert_eq!(gcd_it.next(), None);
    }
}