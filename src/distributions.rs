use std::ops::{AddAssign, RemAssign, SubAssign};

use rand::{seq::SliceRandom, Rng};

pub trait InputDistribution<T> {
    fn generate(rng: &mut impl Rng, len: usize) -> Vec<T>;
    fn name() -> String;
}

pub struct Sorted;
impl<T: num::PrimInt + AddAssign> InputDistribution<T> for Sorted {
    fn name() -> String {
        "Sorted".to_string()
    }
    fn generate(_: &mut impl Rng, len: usize) -> Vec<T> {
        (0..len)
            .scan(T::zero(), |acc, _| {
                let v = *acc;
                *acc += T::one();
                Some(v)
            })
            .collect()
    }
}
pub struct Reverse;
impl<T: num::PrimInt + SubAssign> InputDistribution<T> for Reverse {
    fn name() -> String {
        "Reversed".to_string()
    }
    fn generate(_: &mut impl Rng, len: usize) -> Vec<T> {
        let end = (0..len).fold(T::zero(), |acc, _| acc + T::one());
        (0..len)
            .scan(end, |acc, _| {
                let v = *acc;
                *acc -= T::one();
                Some(v)
            })
            .collect()
    }
}
pub struct AllEqual;
impl<T: num::PrimInt> InputDistribution<T> for AllEqual {
    fn name() -> String {
        "All equal".to_string()
    }
    fn generate(_: &mut impl Rng, len: usize) -> Vec<T> {
        (0..len).map(|_| T::zero()).collect()
    }
}
pub type Shuffled = ShuffledValues<0>;

pub struct ShuffledValues<const N: usize>;
impl<T: num::PrimInt + AddAssign + RemAssign, const N: usize> InputDistribution<T>
    for ShuffledValues<N>
{
    fn name() -> String {
        if N == 0 {
            return "Shuffled".to_string();
        }
        format!("Shuffled ({N} values)")
    }
    fn generate(rng: &mut impl Rng, len: usize) -> Vec<T> {
        let mut v = Sorted::generate(rng, len);
        if N != 0 {
            let m = (0..N).fold(T::zero(), |acc, _| acc + T::one());
            v.iter_mut().for_each(|x| *x %= m);
        }
        v.shuffle(rng);
        v
    }
}

pub struct AscendingDescending;
impl<T: num::PrimInt + AddAssign + SubAssign> InputDistribution<T> for AscendingDescending {
    fn name() -> String {
        "Asc+Dsc".to_string()
    }
    fn generate(_: &mut impl Rng, len: usize) -> Vec<T> {
        let end = (0..len).fold(T::zero(), |acc, _| acc + T::one());
        (0..len / 2)
            .scan(T::zero(), |acc, _| {
                let v = *acc;
                *acc += T::one();
                Some(v)
            })
            .chain((len / 2..len).scan(end, |acc, _| {
                let v = *acc;
                *acc -= T::one();
                Some(v)
            }))
            .collect()
    }
}

pub struct PushFront;
impl<T: num::PrimInt + AddAssign> InputDistribution<T> for PushFront {
    fn name() -> String {
        "Push front int".to_string()
    }
    fn generate(rng: &mut impl Rng, len: usize) -> Vec<T> {
        let mut v = Sorted::generate(rng, len);
        if len == 0 {
            return v;
        }
        let i = v.remove(0);
        v.push(i);
        v
    }
}

pub struct PushMiddle;
impl<T: num::PrimInt + AddAssign> InputDistribution<T> for PushMiddle {
    fn name() -> String {
        "Push middle int".to_string()
    }
    fn generate(rng: &mut impl Rng, len: usize) -> Vec<T> {
        let mut v = Sorted::generate(rng, len);
        if len == 0 {
            return v;
        }
        let i = v.remove(len / 2);
        v.push(i);
        v
    }
}

pub struct Uniform;
impl<T: num::PrimInt> InputDistribution<T> for Uniform
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn name() -> String {
        "Uniform".to_string()
    }
    fn generate(rng: &mut impl Rng, len: usize) -> Vec<T> {
        (0..len).map(|_| rng.gen()).collect()
    }
}
